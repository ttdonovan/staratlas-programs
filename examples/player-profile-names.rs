use anchor_lang::{AnchorDeserialize, Discriminator};
use dotenv::dotenv;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey};

use staratlas_player_profile::{
    ID as PLAYER_PROFILE_ID,
    state::{PlayerName, Profile},
};

fn get_program_accounts<T: Discriminator>(
    client: &RpcClient,
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
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

    let accounts = client.get_program_accounts_with_config(&PLAYER_PROFILE_ID, config)?;
    dbg!(&accounts.len());

    Ok(accounts)
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let rpc_url = dotenv::var("RPC")?;
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let profile_accounts = get_program_accounts::<Profile>(&client)?;
    let player_name_accounts = get_program_accounts::<PlayerName>(&client)?;

    for (pubkey, _profile) in profile_accounts.iter() {
        let (address, _) =
            Pubkey::find_program_address(&[b"player_name", pubkey.as_ref()], &PLAYER_PROFILE_ID);

        if let Some((_, player_name)) = player_name_accounts
            .iter()
            .find(|(pubkey, _)| pubkey == &address)
        {
            // get all the account data as "reader"
            let mut reader = player_name.data.as_slice();
            dbg!(reader.len());

            // first deserialize the program account for PlayerName
            let _ = PlayerName::deserialize_reader(&mut reader)?;
            dbg!(reader.len());

            // TODO: what these 8 bytes represent...
            let skip = u64::deserialize_reader(&mut reader)?;
            dbg!(skip);

            // the final remaining account data from the slice is the "name"
            let lossy_name = String::from_utf8_lossy(reader);
            dbg!(&lossy_name);

            println!(
                "{} - {} - {}",
                pubkey.to_string(),
                skip,
                lossy_name.to_string()
            );
        };

        continue;
    }

    Ok(())
}
