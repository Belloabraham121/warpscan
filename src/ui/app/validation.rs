//! Input validation utilities

/// Check if input looks like an Ethereum address
pub fn is_address(input: &str) -> bool {
    input.starts_with("0x")
        && input.len() == 42
        && input[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Check if input looks like a transaction hash
pub fn is_transaction_hash(input: &str) -> bool {
    input.starts_with("0x")
        && input.len() == 66
        && input[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Check if input looks like a block number
pub fn is_block_number(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_digit())
}
