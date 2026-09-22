use formula_check::modular_arithmetic::{ModularOperation, check_modular_integer_result};
use formula_check::rsa_public::{RsaPublicOperation, check_rsa_public_result};
use formula_check::verifier_equivalence::{
    MontgomeryProductOperation, RsaType1VerifierOperation, Type1PayloadOperation,
    check_montgomery_product_result, check_rsa_type1_payload, check_type1_payload,
};
use num_bigint::BigInt;
use std::collections::BTreeMap;
use std::io::{self, Read};
use std::process::ExitCode;
use std::str::FromStr;

const PROTOCOL_SCHEMA: &str = "formula-verifier-runtime-v1";
const FROZEN_VERIFIER_EQUIVALENCE_HEAD: &str = "944d7d5c6a0094b7dd623519b6c1ff3c899188b1";
const MAX_STDIN_BYTES: u64 = 1_048_576;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CliErrorKind {
    Protocol,
    Verification,
}

#[derive(Debug)]
struct CliError {
    kind: CliErrorKind,
    code: &'static str,
    detail: String,
}

impl CliError {
    fn protocol(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            kind: CliErrorKind::Protocol,
            code,
            detail: detail.into(),
        }
    }

    fn verification(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            kind: CliErrorKind::Verification,
            code,
            detail: detail.into(),
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(lines) => {
            emit("status", "pass");
            for (key, value) in lines {
                emit(&key, &value);
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            emit("status", "fail");
            emit(
                "error_kind",
                match error.kind {
                    CliErrorKind::Protocol => "protocol",
                    CliErrorKind::Verification => "verification",
                },
            );
            emit("error_code", error.code);
            emit("detail", &sanitize_output(&error.detail));
            match error.kind {
                CliErrorKind::Protocol => ExitCode::from(64),
                CliErrorKind::Verification => ExitCode::from(2),
            }
        }
    }
}

fn run() -> Result<Vec<(String, String)>, CliError> {
    let fields = read_request()?;
    let command = required(&fields, "command")?;

    match command {
        "identity" => identity(),
        "powmod" => verify_powmod(&fields),
        "rsa-public" => verify_rsa_public(&fields),
        "montgomery" => verify_montgomery(&fields),
        "type1" => verify_type1(&fields),
        "rsa-type1" => verify_rsa_type1(&fields),
        other => Err(CliError::protocol(
            "unsupported-command",
            format!("unsupported command: {other}"),
        )),
    }
}

fn identity() -> Result<Vec<(String, String)>, CliError> {
    Ok(vec![
        pair("schema", PROTOCOL_SCHEMA),
        pair("mode", "verification-only"),
        pair(
            "verifier_equivalence_frozen_head",
            FROZEN_VERIFIER_EQUIVALENCE_HEAD,
        ),
        pair("signing_supported", "false"),
        pair("authorization_generation_supported", "false"),
        pair("private_key_operations_supported", "false"),
        pair("commands", "powmod,rsa-public,montgomery,type1,rsa-type1"),
    ])
}

fn verify_powmod(fields: &BTreeMap<String, String>) -> Result<Vec<(String, String)>, CliError> {
    let operation = ModularOperation::PowMod {
        base: decimal(fields, "base")?,
        exponent: decimal(fields, "exponent")?,
        modulus: decimal(fields, "modulus")?,
    };
    let producer = required(fields, "producer_decimal")?;
    let receipt = check_modular_integer_result(&operation, producer)
        .map_err(|error| CliError::verification("powmod-rejected", format!("{error:?}")))?;

    Ok(vec![
        pair("operation_digest", receipt.operation_digest().as_str()),
        pair("evidence_digest", receipt.evidence_digest().as_str()),
        pair("result_decimal", receipt.result_decimal()),
    ])
}

fn verify_rsa_public(fields: &BTreeMap<String, String>) -> Result<Vec<(String, String)>, CliError> {
    let operation = RsaPublicOperation::new(
        decimal(fields, "representative")?,
        decimal(fields, "exponent")?,
        decimal(fields, "modulus")?,
    );
    let producer_decimal = required(fields, "producer_decimal")?;
    let producer_hex = required(fields, "producer_hex")?;
    let receipt = check_rsa_public_result(&operation, producer_decimal, producer_hex)
        .map_err(|error| CliError::verification("rsa-public-rejected", format!("{error:?}")))?;

    Ok(vec![
        pair("operation_digest", receipt.operation_digest().as_str()),
        pair("evidence_digest", receipt.evidence_digest().as_str()),
        pair("result_decimal", receipt.result_decimal()),
        pair("result_hex", receipt.result_hex()),
        pair("width_bytes", receipt.width_bytes().to_string()),
    ])
}

fn verify_montgomery(fields: &BTreeMap<String, String>) -> Result<Vec<(String, String)>, CliError> {
    let operation = MontgomeryProductOperation::new(
        decimal(fields, "lhs")?,
        decimal(fields, "rhs")?,
        decimal(fields, "modulus")?,
        positive_usize(fields, "radix_bits")?,
    );
    let producer = required(fields, "producer_decimal")?;
    let receipt = check_montgomery_product_result(&operation, producer)
        .map_err(|error| CliError::verification("montgomery-rejected", format!("{error:?}")))?;

    Ok(vec![
        pair("operation_digest", receipt.operation_digest().as_str()),
        pair("evidence_digest", receipt.evidence_digest().as_str()),
        pair("producer_decimal", receipt.producer_decimal()),
        pair("normalized_decimal", receipt.normalized_decimal()),
        pair(
            "required_subtraction",
            if receipt.required_subtraction() {
                "true"
            } else {
                "false"
            },
        ),
    ])
}

fn verify_type1(fields: &BTreeMap<String, String>) -> Result<Vec<(String, String)>, CliError> {
    let operation = Type1PayloadOperation::new(
        required(fields, "block_hex")?,
        positive_usize(fields, "width_bytes")?,
        required_allow_empty(fields, "expected_payload_hex")?,
    );
    let receipt = check_type1_payload(&operation)
        .map_err(|error| CliError::verification("type1-rejected", format!("{error:?}")))?;

    Ok(vec![
        pair("operation_digest", receipt.operation_digest().as_str()),
        pair("evidence_digest", receipt.evidence_digest().as_str()),
        pair("payload_hex", receipt.payload_hex()),
        pair("padding_bytes", receipt.padding_bytes().to_string()),
    ])
}

fn verify_rsa_type1(fields: &BTreeMap<String, String>) -> Result<Vec<(String, String)>, CliError> {
    let operation = RsaType1VerifierOperation::new(
        decimal(fields, "representative")?,
        decimal(fields, "exponent")?,
        decimal(fields, "modulus")?,
        required_allow_empty(fields, "expected_payload_hex")?,
    );
    let receipt = check_rsa_type1_payload(&operation)
        .map_err(|error| CliError::verification("rsa-type1-rejected", format!("{error:?}")))?;

    Ok(vec![
        pair("operation_digest", receipt.operation_digest().as_str()),
        pair("evidence_digest", receipt.evidence_digest().as_str()),
        pair(
            "recovered_block_digest",
            receipt.recovered_block_digest().as_str(),
        ),
        pair("payload_hex", receipt.payload_hex()),
        pair("padding_bytes", receipt.padding_bytes().to_string()),
    ])
}

fn read_request() -> Result<BTreeMap<String, String>, CliError> {
    let mut input = String::new();
    io::stdin()
        .take(MAX_STDIN_BYTES + 1)
        .read_to_string(&mut input)
        .map_err(|error| CliError::protocol("stdin-read", error.to_string()))?;

    if input.len() as u64 > MAX_STDIN_BYTES {
        return Err(CliError::protocol(
            "request-too-large",
            "stdin exceeds 1048576 bytes",
        ));
    }

    let mut fields = BTreeMap::new();
    for (index, raw_line) in input.lines().enumerate() {
        if raw_line.is_empty() {
            continue;
        }
        let (key, value) = raw_line.split_once('=').ok_or_else(|| {
            CliError::protocol(
                "malformed-line",
                format!("line {} does not contain '='", index + 1),
            )
        })?;
        if !valid_key(key) {
            return Err(CliError::protocol(
                "invalid-key",
                format!("invalid key on line {}", index + 1),
            ));
        }
        if value.contains('\r') || value.contains('\n') {
            return Err(CliError::protocol(
                "invalid-value",
                format!("invalid value on line {}", index + 1),
            ));
        }
        if fields.insert(key.to_owned(), value.to_owned()).is_some() {
            return Err(CliError::protocol(
                "duplicate-key",
                format!("duplicate key: {key}"),
            ));
        }
    }
    Ok(fields)
}

fn valid_key(key: &str) -> bool {
    !key.is_empty()
        && key.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
        })
}

fn required<'a>(fields: &'a BTreeMap<String, String>, key: &str) -> Result<&'a str, CliError> {
    let value = required_allow_empty(fields, key)?;
    if value.is_empty() {
        return Err(CliError::protocol(
            "empty-field",
            format!("field must not be empty: {key}"),
        ));
    }
    Ok(value)
}

fn required_allow_empty<'a>(
    fields: &'a BTreeMap<String, String>,
    key: &str,
) -> Result<&'a str, CliError> {
    fields
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| CliError::protocol("missing-field", format!("missing field: {key}")))
}

fn decimal(fields: &BTreeMap<String, String>, key: &str) -> Result<BigInt, CliError> {
    let value = required(fields, key)?;
    if !canonical_decimal(value) {
        return Err(CliError::protocol(
            "malformed-decimal",
            format!("non-canonical decimal field: {key}"),
        ));
    }
    BigInt::from_str(value)
        .map_err(|_| CliError::protocol("malformed-decimal", format!("invalid decimal: {key}")))
}

fn canonical_decimal(value: &str) -> bool {
    if value == "0" {
        return true;
    }
    if let Some(rest) = value.strip_prefix('-') {
        return !rest.is_empty()
            && !rest.starts_with('0')
            && rest.bytes().all(|byte| byte.is_ascii_digit());
    }
    !value.starts_with('0') && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn positive_usize(fields: &BTreeMap<String, String>, key: &str) -> Result<usize, CliError> {
    let value = required(fields, key)?;
    if value.starts_with('+')
        || value.starts_with('-')
        || (value.len() > 1 && value.starts_with('0'))
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(CliError::protocol(
            "malformed-usize",
            format!("non-canonical integer field: {key}"),
        ));
    }
    let parsed = value
        .parse::<usize>()
        .map_err(|_| CliError::protocol("malformed-usize", format!("invalid integer: {key}")))?;
    if parsed == 0 {
        return Err(CliError::protocol(
            "zero-usize",
            format!("field must be positive: {key}"),
        ));
    }
    Ok(parsed)
}

fn pair(key: impl Into<String>, value: impl Into<String>) -> (String, String) {
    (key.into(), value.into())
}

fn emit(key: &str, value: &str) {
    println!("{key}={value}");
}

fn sanitize_output(value: &str) -> String {
    value.replace(['\r', '\n'], " ")
}
