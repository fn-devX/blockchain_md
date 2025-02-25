use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;
use hex;

pub fn generate_keys() -> (String, String) {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let private_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &private_key);
    (hex::encode(private_key.secret_bytes()), hex::encode(public_key.serialize()))
}
