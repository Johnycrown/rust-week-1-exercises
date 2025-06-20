// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Step 1: Convert hex string to byte vector
    // Step 2: Ensure the transaction has at least 4 bytes
    // Step 3: Extract version (first 4 bytes, little-endian)
    let raw_tx_bytes = hex_to_bytes(raw_tx_hex)?;

    
    if raw_tx_bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    
    let version_bytes = &raw_tx_bytes[0..4];
    let version = u32::from_le_bytes([
        version_bytes[0],
        version_bytes[1],
        version_bytes[2],
        version_bytes[3],
    ]);

    Ok(version)
}


fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex decode error".to_string());
    }

    let mut bytes = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        let byte = u8::from_str_radix(byte_str, 16)
            .map_err(|e| format!("Invalid hex at position {}: {}", i, e))?;
        bytes.push(byte);
    }

    Ok(bytes)
}
