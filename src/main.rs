use bdk::{
    SyncOptions,
    blockchain::ElectrumBlockchain,
    bitcoin::Network,
    database::MemoryDatabase,
    electrum_client::Client,
    wallet::{Wallet, AddressIndex},
};


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

    Ok(())
}
