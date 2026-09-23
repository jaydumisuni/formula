//! Core durable identity types for Formula.
//!
//! P1 is implemented in bounded slices. This slice defines canonical encoding
//! primitives plus the structural identity inputs for `Entity`. It does not
//! publish authority, hash artifacts, or claim completion of Gate P1.

use sha2::{Digest, Sha256};

/// Canonical structural encoding version frozen for the first P1 implementation.
pub const CANONICAL_ENCODING_V1: u16 = 1;

/// Exact 256-bit content/structural digest value.
///
/// Computing the digest is a later P1 slice; this type prevents semantic
/// references from being represented as paths, timestamps, or cache keys.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArtifactDigest([u8; 32]);

impl ArtifactDigest {
    /// Construct an exact digest from its canonical 32-byte representation.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Return the exact canonical digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Compute a Formula structural digest from already-canonical bytes.
    #[must_use]
    pub fn sha256(canonical_bytes: &[u8]) -> Self {
        let digest = Sha256::digest(canonical_bytes);
        let mut bytes = [0_u8; 32];
        bytes.copy_from_slice(&digest);
        Self(bytes)
    }

    /// Lowercase hexadecimal form used by durable manifests and evidence.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut out = String::with_capacity(64);
        for byte in self.0 {
            use core::fmt::Write as _;
            write!(&mut out, "{byte:02x}").expect("writing to String cannot fail");
        }
        out
    }
}

/// Durable structural identity inputs for a mathematical Entity.
///
/// Machine-local paths, wall-clock timestamps, scheduler state, process IDs,
/// cache keys and hardware timing are deliberately absent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entity {
    schema_version: u16,
    kind: String,
    exact_structure: Vec<u8>,
    referenced_entities: Vec<ArtifactDigest>,
    theory_context: Vec<ArtifactDigest>,
}

impl Entity {
    /// Build an Entity from semantic identity inputs only.
    #[must_use]
    pub fn new(
        kind: impl Into<String>,
        exact_structure: impl Into<Vec<u8>>,
        referenced_entities: Vec<ArtifactDigest>,
        theory_context: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            schema_version: CANONICAL_ENCODING_V1,
            kind: kind.into(),
            exact_structure: exact_structure.into(),
            referenced_entities,
            theory_context,
        }
    }

    /// Return the versioned canonical structural bytes used by the digest layer.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = CanonicalWriter::new(b"formula.entity");
        out.u16(self.schema_version);
        out.bytes(self.kind.as_bytes());
        out.bytes(&self.exact_structure);
        out.digest_list(&self.referenced_entities);
        out.digest_list(&self.theory_context);
        out.finish()
    }

    /// SHA-256 over the versioned canonical structural encoding.
    #[must_use]
    pub fn structural_digest(&self) -> ArtifactDigest {
        ArtifactDigest::sha256(&self.canonical_bytes())
    }
}

/// Durable structural identity inputs for a relation between entities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Relation {
    schema_version: u16,
    kind: String,
    members: Vec<ArtifactDigest>,
}

impl Relation {
    #[must_use]
    pub fn new(kind: impl Into<String>, members: Vec<ArtifactDigest>) -> Self {
        Self {
            schema_version: CANONICAL_ENCODING_V1,
            kind: kind.into(),
            members,
        }
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = CanonicalWriter::new(b"formula.relation");
        out.u16(self.schema_version);
        out.bytes(self.kind.as_bytes());
        out.digest_list(&self.members);
        out.finish()
    }

    /// SHA-256 over the versioned canonical structural encoding.
    #[must_use]
    pub fn structural_digest(&self) -> ArtifactDigest {
        ArtifactDigest::sha256(&self.canonical_bytes())
    }
}

/// Durable structural identity inputs for a world of entities and relations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct World {
    schema_version: u16,
    entities: Vec<ArtifactDigest>,
    relations: Vec<ArtifactDigest>,
    theory_context: Vec<ArtifactDigest>,
}

impl World {
    #[must_use]
    pub fn new(
        entities: Vec<ArtifactDigest>,
        relations: Vec<ArtifactDigest>,
        theory_context: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            schema_version: CANONICAL_ENCODING_V1,
            entities,
            relations,
            theory_context,
        }
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = CanonicalWriter::new(b"formula.world");
        out.u16(self.schema_version);
        out.digest_list(&self.entities);
        out.digest_list(&self.relations);
        out.digest_list(&self.theory_context);
        out.finish()
    }

    /// SHA-256 over the versioned canonical structural encoding.
    #[must_use]
    pub fn structural_digest(&self) -> ArtifactDigest {
        ArtifactDigest::sha256(&self.canonical_bytes())
    }
}

/// Durable structural identity inputs for a mathematical judgement.
///
/// Evidence, timestamps and execution metadata are intentionally excluded; a
/// judgement identifies only the proposition asserted inside an exact world.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Judgement {
    schema_version: u16,
    world: ArtifactDigest,
    kind: String,
    proposition: Vec<u8>,
    dependencies: Vec<ArtifactDigest>,
}

/// Metadata describing evidence for a judgement without making the evidence
/// itself part of the judgement's structural identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceEnvelopeMetadata {
    schema_version: u16,
    judgement: ArtifactDigest,
    evidence_kind: String,
    payload: ArtifactDigest,
    checker: ArtifactDigest,
}

impl EvidenceEnvelopeMetadata {
    #[must_use]
    pub fn new(
        judgement: ArtifactDigest,
        evidence_kind: impl Into<String>,
        payload: ArtifactDigest,
        checker: ArtifactDigest,
    ) -> Self {
        Self {
            schema_version: CANONICAL_ENCODING_V1,
            judgement,
            evidence_kind: evidence_kind.into(),
            payload,
            checker,
        }
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = CanonicalWriter::new(b"formula.evidence-envelope-metadata");
        out.u16(self.schema_version);
        out.digest(self.judgement);
        out.bytes(self.evidence_kind.as_bytes());
        out.digest(self.payload);
        out.digest(self.checker);
        out.finish()
    }

    #[must_use]
    pub fn structural_digest(&self) -> ArtifactDigest {
        ArtifactDigest::sha256(&self.canonical_bytes())
    }
}

impl Judgement {
    #[must_use]
    pub fn new(
        world: ArtifactDigest,
        kind: impl Into<String>,
        proposition: impl Into<Vec<u8>>,
        dependencies: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            schema_version: CANONICAL_ENCODING_V1,
            world,
            kind: kind.into(),
            proposition: proposition.into(),
            dependencies,
        }
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = CanonicalWriter::new(b"formula.judgement");
        out.u16(self.schema_version);
        out.digest(self.world);
        out.bytes(self.kind.as_bytes());
        out.bytes(&self.proposition);
        out.digest_list(&self.dependencies);
        out.finish()
    }

    #[must_use]
    pub fn structural_digest(&self) -> ArtifactDigest {
        ArtifactDigest::sha256(&self.canonical_bytes())
    }
}

struct CanonicalWriter {
    bytes: Vec<u8>,
}

impl CanonicalWriter {
    fn new(domain: &[u8]) -> Self {
        let mut writer = Self { bytes: Vec::new() };
        writer.bytes(domain);
        writer
    }

    fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn bytes(&mut self, value: &[u8]) {
        self.u64(value.len() as u64);
        self.bytes.extend_from_slice(value);
    }

    fn digest(&mut self, value: ArtifactDigest) {
        self.bytes.extend_from_slice(value.as_bytes());
    }

    fn digest_list(&mut self, values: &[ArtifactDigest]) {
        self.u64(values.len() as u64);
        for value in values {
            self.bytes.extend_from_slice(value.as_bytes());
        }
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(byte: u8) -> ArtifactDigest {
        ArtifactDigest::from_bytes([byte; 32])
    }

    #[test]
    fn same_semantic_entity_has_identical_canonical_encoding() {
        let left = Entity::new(
            "integer",
            b"42".to_vec(),
            vec![digest(1), digest(2)],
            vec![digest(9)],
        );
        let right = Entity::new(
            "integer",
            b"42".to_vec(),
            vec![digest(1), digest(2)],
            vec![digest(9)],
        );

        assert_eq!(left.canonical_bytes(), right.canonical_bytes());
    }

    #[test]
    fn semantic_structure_change_changes_canonical_encoding() {
        let left = Entity::new("integer", b"42".to_vec(), vec![], vec![]);
        let right = Entity::new("integer", b"43".to_vec(), vec![], vec![]);

        assert_ne!(left.canonical_bytes(), right.canonical_bytes());
    }

    #[test]
    fn referenced_entity_order_is_explicit_semantic_input() {
        let left = Entity::new(
            "application",
            b"f(x,y)".to_vec(),
            vec![digest(1), digest(2)],
            vec![],
        );
        let right = Entity::new(
            "application",
            b"f(x,y)".to_vec(),
            vec![digest(2), digest(1)],
            vec![],
        );

        assert_ne!(left.canonical_bytes(), right.canonical_bytes());
    }

    #[test]
    fn theory_context_is_part_of_structural_identity() {
        let left = Entity::new("symbol", b"x".to_vec(), vec![], vec![digest(3)]);
        let right = Entity::new("symbol", b"x".to_vec(), vec![], vec![digest(4)]);

        assert_ne!(left.canonical_bytes(), right.canonical_bytes());
    }

    #[test]
    fn machine_local_metadata_has_no_encoding_surface() {
        let entity = Entity::new("integer", b"42".to_vec(), vec![], vec![]);
        let before = entity.canonical_bytes();

        let _temporary_path = "/tmp/formula-run-123";
        let _wall_clock = "2026-09-23T02:00:00+02:00";
        let _process_id = 4242_u32;
        let _scheduler_order = 17_u64;

        assert_eq!(before, entity.canonical_bytes());
    }

    #[test]
    fn relation_member_order_is_structural() {
        let left = Relation::new("application", vec![digest(1), digest(2)]);
        let right = Relation::new("application", vec![digest(2), digest(1)]);
        assert_ne!(left.canonical_bytes(), right.canonical_bytes());
    }

    #[test]
    fn world_separates_entity_relation_and_theory_domains() {
        let entity_world = World::new(vec![digest(1)], vec![], vec![]);
        let relation_world = World::new(vec![], vec![digest(1)], vec![]);
        let theory_world = World::new(vec![], vec![], vec![digest(1)]);
        assert_ne!(
            entity_world.canonical_bytes(),
            relation_world.canonical_bytes()
        );
        assert_ne!(
            entity_world.canonical_bytes(),
            theory_world.canonical_bytes()
        );
        assert_ne!(
            relation_world.canonical_bytes(),
            theory_world.canonical_bytes()
        );
    }

    #[test]
    fn sha256_wrapper_matches_standard_known_vector() {
        assert_eq!(
            ArtifactDigest::sha256(b"abc").to_hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn independent_semantic_replay_produces_identical_structural_digest() {
        let first = Entity::new(
            "integer",
            b"42".to_vec(),
            vec![digest(1), digest(2)],
            vec![digest(9)],
        );
        let replay = Entity::new(
            "integer",
            b"42".to_vec(),
            vec![digest(1), digest(2)],
            vec![digest(9)],
        );

        assert_eq!(first.structural_digest(), replay.structural_digest());
    }

    #[test]
    fn semantic_change_changes_structural_digest() {
        let left = Entity::new("integer", b"42".to_vec(), vec![], vec![]);
        let right = Entity::new("integer", b"43".to_vec(), vec![], vec![]);

        assert_ne!(left.structural_digest(), right.structural_digest());
    }

    #[test]
    fn artifact_domains_remain_separate_at_digest_layer() {
        let entity = Entity::new("application", b"".to_vec(), vec![digest(1)], vec![]);
        let relation = Relation::new("application", vec![digest(1)]);
        let world = World::new(vec![digest(1)], vec![], vec![]);

        assert_ne!(entity.structural_digest(), relation.structural_digest());
        assert_ne!(entity.structural_digest(), world.structural_digest());
        assert_ne!(relation.structural_digest(), world.structural_digest());
    }

    #[test]
    fn domain_and_length_prefixes_make_field_boundaries_unambiguous() {
        let left = Entity::new("ab", b"c".to_vec(), vec![], vec![]);
        let right = Entity::new("a", b"bc".to_vec(), vec![], vec![]);

        assert_ne!(left.canonical_bytes(), right.canonical_bytes());
    }
    #[test]
    fn judgement_identity_is_world_and_semantics_bound() {
        let a = Judgement::new(digest(1), "equals", b"x=x".to_vec(), vec![digest(2)]);
        let same = Judgement::new(digest(1), "equals", b"x=x".to_vec(), vec![digest(2)]);
        let other_world = Judgement::new(digest(3), "equals", b"x=x".to_vec(), vec![digest(2)]);
        assert_eq!(a.structural_digest(), same.structural_digest());
        assert_ne!(a.structural_digest(), other_world.structural_digest());
    }
    #[test]
    fn evidence_metadata_is_bound_to_judgement_payload_and_checker() {
        let base = EvidenceEnvelopeMetadata::new(digest(1), "proof", digest(2), digest(3));
        let same = EvidenceEnvelopeMetadata::new(digest(1), "proof", digest(2), digest(3));
        let other_payload = EvidenceEnvelopeMetadata::new(digest(1), "proof", digest(4), digest(3));

        assert_eq!(base.structural_digest(), same.structural_digest());
        assert_ne!(base.structural_digest(), other_payload.structural_digest());
    }
}
