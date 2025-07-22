use wasm_bindgen::prelude::wasm_bindgen;
use dapa_common::{crypto::KeyPair as KeyPairImpl, serializer::Serializer};
use dapa_wallet::mnemonics;

#[wasm_bindgen]
pub struct KeyPair {
    inner: KeyPairImpl,
    mainnet: bool
}

#[wasm_bindgen]
impl KeyPair {
    #[wasm_bindgen(constructor)]
    pub fn new(mainnet: bool) -> Self {
        let inner = KeyPairImpl::new();
        Self { inner, mainnet }
    }

    /// Returns the secret key of the keypair in hex format.
    pub fn secret(&self) -> String {
        self.inner.get_private_key().to_hex()
    }

    /// Generate the seed for the private key.
    /// The seed is a list of words that can be used to recover the private key.
    /// The language_id is the index of the language in the list of supported languages.
    pub fn seed(&self, language_id: usize) -> Result<Vec<String>, String> {
        mnemonics::key_to_words(self.inner.get_private_key(), language_id)
            .map_err(|e| e.to_string())
    }

    /// Returns the address of the keypair.
    pub fn address(&self) -> String {
        self.inner.get_public_key().to_address(self.mainnet).to_string()
    }
}

/// Get all the languages supported by the library.
#[wasm_bindgen]
pub fn get_languages() -> Vec<String> {
    mnemonics::LANGUAGES
        .iter()
        .map(|l| l.get_name().to_string())
        .collect()
}