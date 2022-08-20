use cosmrs::{bip32, crypto};

pub fn mnemonic_to_private_key(mnemonic: String, password: &str) -> crypto::secp256k1::SigningKey {
    let seed = bip32::Mnemonic::new(mnemonic, bip32::Language::English)
        .unwrap()
        .to_seed(password);
    let private_key: crypto::secp256k1::SigningKey = bip32::XPrv::new(seed).unwrap().into();

    private_key
}

pub fn private_to_pub_and_account(
    sender_private_key: &crypto::secp256k1::SigningKey,
    account_id: &str,
) -> (crypto::PublicKey, cosmrs::AccountId) {
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(account_id).unwrap();
    (sender_public_key, sender_account_id)
}
