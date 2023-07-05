use bdk::{
    bitcoin::{Network, secp256k1::{rand, Secp256k1}, util::bip32::{DerivationPath, KeySource}}, keys::{ExtendedKey, DescriptorKey, DerivableKey}, miniscript::Segwitv0,
};
use bdk::keys::bip39::{Mnemonic, Language};
use bdk::keys::DescriptorKey::Secret;
use std::str::FromStr;


// generate fresh descriptor strings and return them via (receive, change) tuple
fn get_descriptors() -> (String, String) {
    // Create a new secp context
    let secp = Secp256k1::new();

    let mut rng = rand::thread_rng();

    // Generate a fresh mnemonic, and from there a privatekey
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 24).unwrap();

    dbg!(&mnemonic.to_string());
    let xkey: ExtendedKey = (mnemonic).into_extended_key().unwrap();
    let xprv = xkey.into_xprv(Network::Testnet).unwrap();

    // Create derived privkey from the above master privkey
    // We use the following derivation paths for receive and change keys
    // receive: "m/84h/1h/0h/0"
    // change: "m/84h/1h/0h/1" 
    let mut keys = Vec::new();

    for path in ["m/84h/1h/0h/0", "m/84h/1h/0h/1"] {
        let deriv_path: DerivationPath = DerivationPath::from_str(path).unwrap();
        let derived_xprv = &xprv.derive_priv(&secp, &deriv_path).unwrap();
        let origin: KeySource = (xprv.fingerprint(&secp), deriv_path);
        let derived_xprv_desc_key: DescriptorKey<Segwitv0> =
        derived_xprv.into_descriptor_key(Some(origin), DerivationPath::default()).unwrap();

        // Wrap the derived key with the wpkh() string to produce a descriptor string
        if let Secret(key, _, _) = derived_xprv_desc_key {
            let mut desc = "wpkh(".to_string();
            desc.push_str(&key.to_string());
            desc.push_str(")");
            keys.push(desc);
        }
    }
    
    // Return the keys as a tuple
    (keys[0].clone(), keys[1].clone())
}


fn main() {
    let (receive_desc, change_desc) = get_descriptors();
    println!("recv: {:#?}, \nchng: {:#?}", receive_desc, change_desc);
}
