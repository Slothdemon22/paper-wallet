use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use web_sys::console;
use sha2::{Sha256, Digest};
use ripemd::{Ripemd160};
use k256::{SecretKey, ecdsa::{SigningKey, VerifyingKey}};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use base58::ToBase58;
use getrandom::getrandom;

// WASM-compatible random number generation workaround
fn secure_random_bytes(len: usize) -> Result<Vec<u8>, JsValue> {
    let mut buffer = vec![0u8; len];
    getrandom(&mut buffer).map_err(|e| JsValue::from_str(&format!("Random generation failed: {}", e)))?;
    Ok(buffer)
}

#[wasm_bindgen]
pub struct DAPAWallet {
    private_key: String,
    public_key: String,
    address: String,
}

#[wasm_bindgen]
impl DAPAWallet {
    #[wasm_bindgen(getter)]
    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> String {
        self.public_key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn address(&self) -> String {
        self.address.clone()
    }
}

// DAPA address generation (similar to Bitcoin but with 'dap' prefix)
fn generate_dapa_address(public_key: &VerifyingKey) -> String {
    // Serialize public key (uncompressed format)
    let pub_key_point = public_key.to_encoded_point(false);
    let pub_key_bytes = pub_key_point.as_bytes();
    
    // SHA256 hash
    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(pub_key_bytes);
    let sha256_result = sha256_hasher.finalize();
    
    // RIPEMD160 hash
    let mut ripemd_hasher = Ripemd160::new();
    ripemd_hasher.update(&sha256_result);
    let ripemd_result = ripemd_hasher.finalize();
    
    // Add DAPA network byte (using 0x1A for 'dap' prefix)
    let mut extended = vec![0x1A];
    extended.extend_from_slice(&ripemd_result);
    
    // Double SHA256 for checksum
    let mut checksum_hasher = Sha256::new();
    checksum_hasher.update(&extended);
    let checksum_hash1 = checksum_hasher.finalize();
    
    let mut checksum_hasher2 = Sha256::new();
    checksum_hasher2.update(&checksum_hash1);
    let checksum_hash2 = checksum_hasher2.finalize();
    
    // Take first 4 bytes as checksum
    extended.extend_from_slice(&checksum_hash2[..4]);
    
    // Base58 encode
    let base58_addr = extended.to_base58();
    
    // Ensure it starts with 'dap' by adjusting the encoding
    format!("dap{}", &base58_addr[3..])
}

#[wasm_bindgen]
pub fn generate_wallet() -> Result<DAPAWallet, JsValue> {
    console::log_1(&"Generating new DAPA wallet...".into());
    
    // Generate secure random private key (32 bytes)
    let private_key_bytes = secure_random_bytes(32)?;
    
    // Create signing key from random bytes
    let signing_key = SigningKey::from_slice(&private_key_bytes)
        .map_err(|e| JsValue::from_str(&format!("Invalid private key: {}", e)))?;
    
    // Generate public key
    let public_key = VerifyingKey::from(&signing_key);
    
    // Generate DAPA address
    let address = generate_dapa_address(&public_key);
    
    // Convert keys to hex strings
    let private_key_hex = hex::encode(&private_key_bytes);
    let public_key_hex = hex::encode(public_key.to_encoded_point(true).as_bytes());
    
    console::log_1(&format!("Generated address: {}", address).into());
    
    Ok(DAPAWallet {
        private_key: private_key_hex,
        public_key: public_key_hex,
        address,
    })
}

#[wasm_bindgen]
pub fn validate_dapa_address(address: &str) -> bool {
    // Simple validation - DAPA addresses should start with 'dap'
    address.starts_with("dap") && address.len() >= 26 && address.len() <= 35
}

// Helper function to convert private key to WIF format
#[wasm_bindgen]
pub fn private_key_to_wif(private_key_hex: &str) -> Result<String, JsValue> {
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|_| JsValue::from_str("Invalid hex private key"))?;
    
    if private_key_bytes.len() != 32 {
        return Err(JsValue::from_str("Private key must be 32 bytes"));
    }
    
    // Add network byte for DAPA (using 0x9A for WIF)
    let mut extended = vec![0x9A];
    extended.extend_from_slice(&private_key_bytes);
    extended.push(0x01); // Compressed flag
    
    // Double SHA256 for checksum
    let mut hasher1 = Sha256::new();
    hasher1.update(&extended);
    let hash1 = hasher1.finalize();
    
    let mut hasher2 = Sha256::new();
    hasher2.update(&hash1);
    let hash2 = hasher2.finalize();
    
    // Add checksum
    extended.extend_from_slice(&hash2[..4]);
    
    Ok(extended.to_base58())
}

// Hex encoding helper (since we can't use external hex crate easily)
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
    
    pub fn decode(hex_str: &str) -> Result<Vec<u8>, &'static str> {
        if hex_str.len() % 2 != 0 {
            return Err("Invalid hex length");
        }
        
        (0..hex_str.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex_str[i..i + 2], 16)
                    .map_err(|_| "Invalid hex character")
            })
            .collect()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"DAPA Wallet Generator initialized!".into());
}