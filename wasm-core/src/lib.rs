use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct AttestationReport {
    pub timestamp: u64,
    pub total_assets: f64,
    pub total_liabilities: f64,
    pub solvency_ratio: f64,
    pub is_solvent: bool,
    pub hash_proof: String,
}

#[wasm_bindgen]
pub fn verify_solvency(assets: f64, liabilities: f64) -> Result<JsValue, JsValue> {
    let is_solvent = assets >= liabilities;
    let ratio = if liabilities > 0.0 { assets / liabilities } else { 0.0 };
    
    // In a real production scenario, we would get a real timestamp
    // For Wasm on Edge, we might pass the timestamp from JS or use Date.now()
    let timestamp = js_sys::Date::now() as u64;

    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}{}", timestamp, assets, liabilities, is_solvent).as_bytes());
    let hash_proof = hex::encode(hasher.finalize());

    let report = AttestationReport {
        timestamp,
        total_assets: assets,
        total_liabilities: liabilities,
        solvency_ratio: ratio,
        is_solvent,
        hash_proof,
    };

    serde_json::to_value(&report)
        .map(|v| v.into())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn generate_signed_attestation(report_json: &str, private_key: &str) -> String {
    // This is a placeholder for actual Ed25519 signing logic
    // For now, we'll simulate a signature using the private key and report hash
    let mut hasher = Sha256::new();
    hasher.update(report_json.as_bytes());
    hasher.update(private_key.as_bytes());
    let signature = hex::encode(hasher.finalize());
    
    format!("ATTEST-SIG:{}", signature)
}
