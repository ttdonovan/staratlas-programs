use anchor_lang::{prelude::Pubkey as AnchorPubkey, AccountDeserialize, InstructionData};
use litesvm::LiteSVM;
use solana_sdk::{
    feature_set::FeatureSet,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};

use staratlas_player_profile::{instruction::CreateProfile, typedefs::AddKeyInput};
use staratlas_profile_faction::{instruction::ChooseFaction, typedefs::Faction};
use staratlas_sage::{instruction::InitGame, state::Game};

const PLAYER_PROFILE_PROGRAM_ID: Pubkey = pubkey!("pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9");
const PROFILE_FACTION_PROGRAM_ID: Pubkey = pubkey!("pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq");
const SAGE_PROGRAM_ID: Pubkey = pubkey!("SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE");

fn read_player_profile_program() -> Vec<u8> {
    let mut bin_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bin_path.push("programs/player-profile/pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9.bin");
    std::fs::read(bin_path).unwrap()
}

fn read_profile_faction() -> Vec<u8> {
    let mut bin_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bin_path.push("programs/profile-faction/pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq.bin");
    std::fs::read(bin_path).unwrap()
}

fn read_sage_program() -> Vec<u8> {
    let mut bin_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bin_path.push("programs/sage/SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE.bin");
    std::fs::read(bin_path).unwrap()
}

#[test]
fn sage_test() {
    let feature_set = FeatureSet::all_enabled();
    let mut svm = LiteSVM::default()
        .with_builtins(Some(feature_set))
        .with_lamports(1_000_000_000_000_000)
        .with_sysvars();

    let funder_kp = Keypair::new();
    let funder_pk = funder_kp.pubkey();

    let wallet_kp = Keypair::new();
    let wallet_pk = wallet_kp.pubkey();

    let player_profile_program_bytes = read_player_profile_program();
    let profile_faction_program_bytes = read_profile_faction();
    let sage_program_bytes = read_sage_program();

    svm.airdrop(&funder_pk, 10_000_000_000).unwrap();
    svm.airdrop(&wallet_pk, 10_000_000_000).unwrap();
    svm.add_program(PLAYER_PROFILE_PROGRAM_ID, &player_profile_program_bytes);
    svm.add_program(PROFILE_FACTION_PROGRAM_ID, &profile_faction_program_bytes);
    svm.add_program(SAGE_PROGRAM_ID, &sage_program_bytes);

    let authority_kp = Keypair::new();
    let authority_pk = authority_kp.pubkey();

    let player_profile_kp = Keypair::new();
    let player_profile_pk = player_profile_kp.pubkey();

    // TODO: create a "super" user profile
    let create_profile_ix = Instruction {
        program_id: PLAYER_PROFILE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(wallet_pk, true), // funder (writable, signer)
            AccountMeta::new(player_profile_pk, true), // profile (writable, signer)
            AccountMeta::new_readonly(system_program::ID, false), // system program
            AccountMeta::new(authority_pk, true), // auth authority (writable, signer)
        ],
        data: CreateProfile {
            _key_permissions: vec![
                // Auth Authority - Full permissions
                AddKeyInput {
                    scope: AnchorPubkey::new_from_array(PLAYER_PROFILE_PROGRAM_ID.to_bytes()),
                    expire_time: -1,
                    permissions: [0xFF, 0xFF, 0, 0, 0, 0, 0, 0], // All permissions enabled
                },
                // // Cargo Superuser
                // AddKeyInput {
                //     scope: AnchorPubkey::new_from_array(CARGO_PROGRAM_ID.to_bytes()),
                //     expire_time: -1,
                //     permissions: [0xFF, 0xFF, 0, 0, 0, 0, 0, 0], // CargoPermissions.all()
                // },
                // // Fleet Superuser
                // AddKeyInput {
                //     scope: AnchorPubkey::new_from_array(SAGE_PROGRAM_ID.to_bytes()),
                //     expire_time: -1,
                //     permissions: [0xFF, 0xFF, 0, 0, 0, 0, 0, 0], // SagePermissions.all()
                // },
                // // Fleet Manager
                // AddKeyInput {
                //     scope: AnchorPubkey::new_from_array(SAGE_PROGRAM_ID.to_bytes()),
                //     expire_time: -1,
                //     permissions: [0x01, 0, 0, 0, 0, 0, 0, 0], // MANAGE_FLEET permission
                // },
                // // Remove Ship Escrow
                // AddKeyInput {
                //     scope: AnchorPubkey::new_from_array(SAGE_PROGRAM_ID.to_bytes()),
                //     expire_time: -1,
                //     permissions: [0x02, 0, 0, 0, 0, 0, 0, 0], // REMOVE_SHIP_ESCROW permission
                // },
                // // Ship Manager
                // AddKeyInput {
                //     scope: AnchorPubkey::new_from_array(SAGE_PROGRAM_ID.to_bytes()),
                //     expire_time: -1,
                //     permissions: [0x04, 0, 0, 0, 0, 0, 0, 0], // MANAGE_SHIP permission
                // },
                // // Fleet Cargo Manager
                // AddKeyInput {
                //     scope: AnchorPubkey::new_from_array(SAGE_PROGRAM_ID.to_bytes()),
                //     expire_time: -1,
                //     permissions: [0x08, 0, 0, 0, 0, 0, 0, 0], // MANAGE_FLEET_CARGO permission
                // },
            ],
            _key_threshold: 1,
        }
        .data(),
    };

    let message = Message::new(&[create_profile_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&wallet_kp, &player_profile_kp, &authority_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    let (faction_pda, _bump) = Pubkey::find_program_address(
        &[b"player_faction", player_profile_pk.as_ref()],
        &PROFILE_FACTION_PROGRAM_ID,
    );

    let choose_faction_ix = Instruction {
        program_id: PROFILE_FACTION_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(authority_pk, true), // pub key: Signer<'info>,
            AccountMeta::new(wallet_pk, true),    // pub funder: Signer<'info>,
            AccountMeta::new(player_profile_pk, false), // pub profile: AccountInfo<'info>,
            AccountMeta::new(faction_pda, false), // pub faction: AccountInfo<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // system program
        ],
        data: ChooseFaction {
            _key_index: 0,
            _faction: Faction::Ustur,
        }
        .data(),
    };

    let message = Message::new(&[choose_faction_ix], Some(&wallet_pk));
    let tx = Transaction::new(&[&authority_kp, &wallet_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // init game
    let game_kp = Keypair::new();
    let game_pk = game_kp.pubkey();

    let init_game_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(authority_pk, true), // pub signer: Signer<'info>,
            AccountMeta::new(player_profile_pk, false), // pub profile: AccountInfo<'info>,
            AccountMeta::new(wallet_pk, true), // pub funder: Signer<'info>,
            AccountMeta::new(game_pk, true), // pub game_id: Signer<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
        ],
        data: InitGame {}.data(),
    };

    let message = Message::new(&[init_game_ix], Some(&wallet_pk));
    let tx = Transaction::new(&[&authority_kp, &wallet_kp, &game_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    let game_acc = svm.get_account(&game_pk).unwrap();
    let game_data = Game::try_deserialize(&mut &game_acc.data[..]).unwrap();

    dbg!(&game_acc);
    dbg!(&game_data.game_state);

    assert!(true);
}
