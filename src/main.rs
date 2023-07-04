use std::str::FromStr;
use bdk::{
    bitcoin::{Address, Network, self},
    blockchain::{ElectrumBlockchain, Blockchain},
    database::MemoryDatabase,
    electrum_client::Client,
    wallet::{AddressIndex, Wallet},
    SyncOptions, SignOptions, FeeRate,
};
use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let external_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
    let internal_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/1/*)";

    let wallet: Wallet<MemoryDatabase> = Wallet::new(
        external_descriptor,
        Some(internal_descriptor),
        Network::Testnet,
        MemoryDatabase::new(),
    )?;

    let address = wallet.get_address(AddressIndex::New)?;
    println!("Generated Address: {}", address);

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);

    wallet.sync(&blockchain, SyncOptions::default())?;

    let balance = wallet.get_balance()?;
    println!("Wallet balance in SAT: {}", balance);

    // let my_address = Address::from_str("tb1q7w0t936xp5p994qx506xj53gjdcmzjr2mkqghn")?;

    // let faucet_address = Address::from_str("mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt")?;

    // let (mut psbt, details) = {
    //     let mut builder =  wallet.build_tx();
    //     builder
    //         .add_recipient(faucet_address.script_pubkey(), 800)
    //         .enable_rbf()
    //         .fee_rate(FeeRate::from_sat_per_vb(2.0));
    //     builder.finish()?
    // };

    // println!("Transaction details: {:#?}", details);

    // let mut tx_builder = wallet.build_tx();
    // tx_builder
    //     .add_recipient(faucet_address.script_pubkey(), balance - 600)
    //     .fee_rate(FeeRate::from_sat_per_vb(2.0))
    //     .enable_rbf();
    // let (mut psbt, tx_details) = tx_builder.finish()?;

    // println!("Transaction details: {:#?}", tx_details);


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
