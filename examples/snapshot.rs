use anchor_lang::Discriminator;
use bincode::{Decode, Encode};
use dotenv::dotenv;
use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use serde::{Deserialize, Serialize};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use staratlas_sage::{
    ID as SAGE_ID,
    state::{
        CraftingInstance, DisbandedFleet, Fleet, FleetShips, Game, GameState, MineItem, Planet,
        PlayerCrewRecord, ProgressionConfig, Resource, SageCrewConfig, SagePlayerProfile, Sector,
        Ship, Star, Starbase, StarbasePlayer, SurveyDataUnitTracker,
    },
};

// Create serializable wrapper types for Solana types
#[derive(Serialize, Deserialize, Encode, Decode, Debug)]
struct SerializablePubkey(String);

#[derive(Serialize, Deserialize, Encode, Decode, Debug)]
struct SerializableAccount {
    lamports: u64,
    data: Vec<u8>,
    owner: SerializablePubkey,
    executable: bool,
    rent_epoch: u64,
}

impl From<Pubkey> for SerializablePubkey {
    fn from(pubkey: Pubkey) -> Self {
        SerializablePubkey(pubkey.to_string())
    }
}

impl TryFrom<SerializablePubkey> for Pubkey {
    type Error = anyhow::Error;

    fn try_from(serializable: SerializablePubkey) -> Result<Self, Self::Error> {
        Ok(Pubkey::from_str_const(&serializable.0))
    }
}

impl From<Account> for SerializableAccount {
    fn from(account: Account) -> Self {
        SerializableAccount {
            lamports: account.lamports,
            data: account.data,
            owner: SerializablePubkey::from(account.owner),
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        }
    }
}

impl TryInto<Account> for SerializableAccount {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Account, Self::Error> {
        Ok(Account {
            lamports: self.lamports,
            data: self.data,
            owner: self.owner.try_into()?,
            executable: self.executable,
            rent_epoch: self.rent_epoch,
        })
    }
}

fn get_program_accounts<T: Discriminator>(
    client: &RpcClient,
) -> anyhow::Result<(&[u8], Vec<(Pubkey, Account)>)> {
    let discrim = T::DISCRIMINATOR;
    let config = RpcProgramAccountsConfig {
        account_config: RpcAccountInfoConfig {
            encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
            0,
            MemcmpEncodedBytes::Bytes(discrim.into()),
        ))]),
        ..Default::default()
    };

    let accounts = client.get_program_accounts_with_config(&SAGE_ID, config)?;
    dbg!(&accounts.len());

    Ok((discrim, accounts))
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let rpc_url = dotenv::var("RPC")?;
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let mut collection = HashMap::new();

    let account_types = [
        get_program_accounts::<CraftingInstance>,
        get_program_accounts::<DisbandedFleet>,
        get_program_accounts::<Fleet>,
        get_program_accounts::<FleetShips>,
        get_program_accounts::<Game>,
        get_program_accounts::<GameState>,
        get_program_accounts::<MineItem>,
        get_program_accounts::<Planet>,
        get_program_accounts::<PlayerCrewRecord>,
        get_program_accounts::<ProgressionConfig>,
        get_program_accounts::<Resource>,
        get_program_accounts::<SageCrewConfig>,
        get_program_accounts::<SagePlayerProfile>,
        get_program_accounts::<Sector>,
        get_program_accounts::<Ship>,
        get_program_accounts::<Star>,
        get_program_accounts::<Starbase>,
        get_program_accounts::<StarbasePlayer>,
        get_program_accounts::<SurveyDataUnitTracker>,
    ];

    for get_accounts in account_types.iter() {
        let (discrim, accounts) = get_accounts(&client)?;
        collection.insert(discrim, accounts);
    }

    // Convert to serializable format
    let mut serializable_collection: HashMap<
        Vec<u8>,
        Vec<(SerializablePubkey, SerializableAccount)>,
    > = HashMap::new();

    for (discrim, accounts) in collection {
        let serializable_accounts = accounts
            .into_iter()
            .map(|(pubkey, account)| {
                (
                    SerializablePubkey::from(pubkey),
                    SerializableAccount::from(account),
                )
            })
            .collect();

        serializable_collection.insert(discrim.to_vec(), serializable_accounts);
    }

    let encoded: Vec<u8> =
        bincode::encode_to_vec(&serializable_collection, bincode::config::standard())?;

    // Compress the encoded data with gzip
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&encoded)?;
    let compressed_data = encoder.finish()?;

    // Write the compressed data to file
    let mut file = File::create("data/sage.bin.gz")?;
    file.write_all(&compressed_data)?;

    println!("Compressed snapshot saved to data/sage.bin.gz");
    println!(
        "Original size: {} bytes, Compressed size: {} bytes",
        encoded.len(),
        compressed_data.len()
    );

    // To decompress and read back later:
    let mut file = File::open("data/sage.bin.gz")?;
    let mut compressed_data = Vec::new();
    file.read_to_end(&mut compressed_data)?;

    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data)?;

    let (_decoded_collection, _): (
        HashMap<Vec<u8>, Vec<(SerializablePubkey, SerializableAccount)>>,
        _,
    ) = bincode::decode_from_slice(&decoded_data, bincode::config::standard())?;

    println!("Successfully decoded the snapshot file");

    Ok(())
}
