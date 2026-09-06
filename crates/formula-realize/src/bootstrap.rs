use formula_core::{
    bootstrap::{
        BOOTSTRAP_CORE_SCHEMA_V1, BootstrapBytecode, BootstrapDecision, BootstrapGeneratorImage,
        BootstrapInstruction, BootstrapProgramSource,
    },
    digest::ArtifactDigest,
};

const FBC1_HEADER: &[u8; 4] = b"FBC1";
const FBC1_CANONICAL_BYTECODE: &[u8; 8] = b"FBC1\x01\x02\x03\x04";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootstrapGenerationError {
    UnsupportedSchema,
    UnsupportedSource,
    IncompleteGeneratorImage,
    ArtifactCannotSeedGenerator,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootstrapExecutionError {
    MalformedOrUnsupportedBytecode,
}

pub fn canonical_generator_image() -> BootstrapGeneratorImage {
    BootstrapGeneratorImage::new(
        BOOTSTRAP_CORE_SCHEMA_V1.into(),
        FBC1_HEADER.to_vec(),
        vec![
            (BootstrapInstruction::LoadActualDigest, 0x01),
            (BootstrapInstruction::LoadExpectedDigest, 0x02),
            (BootstrapInstruction::DigestEq, 0x03),
            (BootstrapInstruction::ReturnDecision, 0x04),
        ],
    )
}

pub fn compile_bootstrap_source(
    source: &BootstrapProgramSource,
) -> Result<BootstrapBytecode, BootstrapGenerationError> {
    validate_canonical_source(source)?;
    Ok(BootstrapBytecode::new(FBC1_CANONICAL_BYTECODE.to_vec()))
}

pub fn generator_image_from_bootstrap_artifact(
    source: &BootstrapProgramSource,
    artifact: &BootstrapBytecode,
) -> Result<BootstrapGeneratorImage, BootstrapGenerationError> {
    validate_canonical_source(source)?;

    let bytes = artifact.bytes();
    if bytes != FBC1_CANONICAL_BYTECODE {
        return Err(BootstrapGenerationError::ArtifactCannotSeedGenerator);
    }

    let opcode_map = source
        .instructions()
        .iter()
        .copied()
        .zip(bytes[FBC1_HEADER.len()..].iter().copied())
        .collect();

    Ok(BootstrapGeneratorImage::new(
        source.schema().into(),
        bytes[..FBC1_HEADER.len()].to_vec(),
        opcode_map,
    ))
}

pub fn rebuild_with_generator_image(
    image: &BootstrapGeneratorImage,
    source: &BootstrapProgramSource,
) -> Result<BootstrapBytecode, BootstrapGenerationError> {
    if source.schema() != image.schema() || image.header() != FBC1_HEADER {
        return Err(BootstrapGenerationError::UnsupportedSchema);
    }
    validate_canonical_source(source)?;

    let mut bytes = image.header().to_vec();
    for instruction in source.instructions() {
        let opcode = image
            .opcode_for(*instruction)
            .ok_or(BootstrapGenerationError::IncompleteGeneratorImage)?;
        bytes.push(opcode);
    }
    Ok(BootstrapBytecode::new(bytes))
}

pub fn execute_bootstrap_bytecode(
    bytecode: &BootstrapBytecode,
    actual: ArtifactDigest,
    expected: ArtifactDigest,
) -> Result<BootstrapDecision, BootstrapExecutionError> {
    if bytecode.bytes() != FBC1_CANONICAL_BYTECODE {
        return Err(BootstrapExecutionError::MalformedOrUnsupportedBytecode);
    }
    Ok(if actual == expected {
        BootstrapDecision::Valid
    } else {
        BootstrapDecision::Reject
    })
}

fn validate_canonical_source(
    source: &BootstrapProgramSource,
) -> Result<(), BootstrapGenerationError> {
    if source.schema() != BOOTSTRAP_CORE_SCHEMA_V1 {
        return Err(BootstrapGenerationError::UnsupportedSchema);
    }
    if source.instructions()
        != [
            BootstrapInstruction::LoadActualDigest,
            BootstrapInstruction::LoadExpectedDigest,
            BootstrapInstruction::DigestEq,
            BootstrapInstruction::ReturnDecision,
        ]
    {
        return Err(BootstrapGenerationError::UnsupportedSource);
    }
    Ok(())
}
