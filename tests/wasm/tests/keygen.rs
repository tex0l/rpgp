//! Wasm runtime smoke-test for keygen.
//!
//! Documented as ✅ in `docs/PLATFORMS.md`, but until this test landed
//! the CI matrix only ran `cargo check` against `wasm32-unknown-unknown`
//! — it never executed any code. Every keygen path eventually calls
//! [`pgp::types::Timestamp::now()`] (key creation, signature creation,
//! etc.), which on `wasm32-unknown-unknown` resolves to std's
//! `unsupported::time::SystemTime::now()` and panics with "time not
//! implemented on this platform". A pure `cargo check` cannot see that.
//!
//! `wasm-pack test --node tests/wasm` actually executes the keygen
//! code paths, so any regression in the wasm time source lights up
//! here.

#![cfg(target_arch = "wasm32")]

use pgp::composed::{EncryptionCaps, KeyType, SecretKeyParamsBuilder, SubkeyParamsBuilder};
use rand::thread_rng;
use wasm_bindgen_test::wasm_bindgen_test;

/// Generates an Ed25519 primary + X25519 encryption subkey — the
/// modern ECC layout used by every recent PGP key. This is the
/// minimal end-to-end exercise of `Timestamp::now()` inside the
/// key/subkey/signature building path.
#[wasm_bindgen_test]
fn generate_ed25519_x25519_key_in_wasm() {
    let mut encrypt_subkey = SubkeyParamsBuilder::default();
    encrypt_subkey
        .key_type(KeyType::X25519)
        .can_sign(false)
        .can_encrypt(EncryptionCaps::All)
        .can_authenticate(false);

    let mut params = SecretKeyParamsBuilder::default();
    params
        .key_type(KeyType::Ed25519)
        .can_certify(true)
        .can_sign(true)
        .can_encrypt(EncryptionCaps::None)
        .primary_user_id("Wasm Test <wasm@example.com>".to_string())
        .subkeys(vec![encrypt_subkey.build().expect("subkey params")]);

    let secret_key_params = params.build().expect("build secret params");
    let _signed = secret_key_params
        .generate(thread_rng())
        .expect("generate signed key");
}
