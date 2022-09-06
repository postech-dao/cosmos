use crate::utils::bip32::ExtendedPrivateKey;
use cosmrs::{bip32, crypto};
use std::error::Error;

pub fn mnemonic_to_private_key(
    mnemonic: String,
    password: &str,
) -> Result<ExtendedPrivateKey<cosmrs::bip32::secp256k1::ecdsa::SigningKey>, Box<dyn Error>> {
    let seed = bip32::Mnemonic::new(mnemonic, bip32::Language::English)?.to_seed(password);
    let private_key = bip32::XPrv::new(seed)?;

    Ok(private_key)
}

pub fn private_to_pub_and_account(
    sender_private_key: &crypto::secp256k1::SigningKey,
    account_id: &str,
) -> Result<(crypto::PublicKey, cosmrs::AccountId), Box<dyn Error>> {
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(account_id)?;
    Ok((sender_public_key, sender_account_id))
}
