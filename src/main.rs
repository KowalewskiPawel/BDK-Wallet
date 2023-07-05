use bdk::{
    bitcoin::{Address, Network},
    blockchain::{Blockchain, ElectrumBlockchain},
    database::MemoryDatabase,
    electrum_client::Client,
    wallet::{AddressIndex, Wallet},
    FeeRate, SignOptions, SyncOptions
};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let receive_descriptor = "";
    let change_descriptor = "";

    let wallet: Wallet<MemoryDatabase> = Wallet::new(
        receive_descriptor,
        Some(change_descriptor),
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

    let faucet_address = Address::from_str("mohjSavDdQYHRYXcS3uS6ttaHP8amyvX78")?;

    let (mut psbt, _details) = {
        let mut builder = wallet.build_tx();
        builder
            .drain_to(address.script_pubkey())
            .add_recipient(faucet_address.script_pubkey(), 800)
            .enable_rbf()
            .fee_rate(FeeRate::from_sat_per_vb(2.0));
        builder.finish()?
    };

    let finalized = wallet.sign(&mut psbt, SignOptions::default())?;
    assert!(finalized, "Tx has not been finalized");
    println!("Transaction Signed: {}", finalized);

    let raw_transaction = psbt.extract_tx();
    let txid = raw_transaction.txid();
    blockchain.broadcast(&raw_transaction)?;
    println!(
        "Transaction sent! TXID: {txid}.\nExplorer URL: https://blockstream.info/testnet/tx/{txid}",
        txid = txid
    );

    Ok(())
}
