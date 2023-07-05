use bdk::{
    bitcoin::{self, Address, Network, secp256k1::{rand, Secp256k1}, util::bip32::{DerivationPath, KeySource}},
    blockchain::{Blockchain, ElectrumBlockchain},
    database::MemoryDatabase,
    electrum_client::Client,
    wallet::{AddressIndex, Wallet},
    FeeRate, SignOptions, SyncOptions, keys::{GeneratedKey, ExtendedKey, DescriptorKey, DerivableKey}, miniscript::Segwitv0,
};
use bdk::keys::bip39::{Mnemonic, Language};
use bdk::keys::DescriptorKey::Secret;
use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;
use std::str::FromStr;


// generate fresh descriptor strings and return them via (receive, change) tuple
fn get_descriptors() -> (String, String) {
    // Create a new secp context
    let secp = Secp256k1::new();
     
    // You can also set a password to unlock the mnemonic
    let password = Some("random password".to_string());

    let mut rng = rand::thread_rng();

    // Generate a fresh mnemonic, and from there a privatekey
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 24).unwrap();

    let xkey: ExtendedKey = (mnemonic, password).into_extended_key().unwrap();
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


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (receive_desc, change_desc) = get_descriptors();
    println!("recv: {:#?}, \nchng: {:#?}", receive_desc, change_desc);
    // let external_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
    // let internal_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/1/*)";

    // let wallet: Wallet<MemoryDatabase> = Wallet::new(
    //     external_descriptor,
    //     Some(internal_descriptor),
    //     Network::Testnet,
    //     MemoryDatabase::new(),
    // )?;

    // let random_address_index = rand::random::<u16>();

    // let address = wallet.get_address(AddressIndex::New)?;
    // println!("Generated Address: {}", address);

    // let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    // let blockchain = ElectrumBlockchain::from(client);

    // wallet.sync(&blockchain, SyncOptions::default())?;

    // let balance = wallet.get_balance()?;
    // println!("Wallet balance in SAT: {}", balance);

    // let my_address = Address::from_str("tb1q7w0t936xp5p994qx506xj53gjdcmzjr2mkqghn")?;

    // let faucet_address = Address::from_str("mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt")?;

    // let (mut psbt, details) = {
    //     let mut builder = wallet.build_tx();
    //     builder
    //         .drain_to(address.script_pubkey())
    //         .add_recipient(faucet_address.script_pubkey(), 800)
    //         .enable_rbf()
    //         .fee_rate(FeeRate::from_sat_per_vb(2.0));
    //     builder.finish()?
    // };

    // let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
    // assert!(finalized, "Tx has not been finalized");
    // println!("Transaction Signed: {}", finalized);

    // let raw_transaction = psbt.extract_tx();
    // let txid = raw_transaction.txid();
    // blockchain.broadcast(&raw_transaction)?;
    // println!(
    //     "Transaction sent! TXID: {txid}.\nExplorer URL: https://blockstream.info/testnet/tx/{txid}",
    //     txid = txid
    // );

    Ok(())
}
