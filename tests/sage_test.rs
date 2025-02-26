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

use staratlas_cargo::{instruction::InitDefinition, typedefs::InitDefinitionInput};
use staratlas_player_profile::{instruction::CreateProfile, typedefs::AddKeyInput};
use staratlas_profile_faction::{instruction::ChooseFaction, typedefs::Faction};
use staratlas_sage::{
    instruction::{
        ActivateGameState, InitGame, InitGameState, RegisterSagePlayerProfile, RegisterSector,
        RegisterStarbase, RegisterStarbasePlayer, UpdateGame, UpdateGameState,
    },
    state::{Game, GameState, Sector},
    typedefs::{
        FleetInput, InitGameStateInput, ManageGameInput, RegisterStarbaseInputUnpacked, SectorRing,
        StarbaseLevelInfoArrayInput, UpdateGameInput, UpdateGameStateInput,
    },
};

const CARGO_PROGRAM_ID: Pubkey = pubkey!("Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk");
const PLAYER_PROFILE_PROGRAM_ID: Pubkey = pubkey!("pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9");
const PROFILE_FACTION_PROGRAM_ID: Pubkey = pubkey!("pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq");
const SAGE_PROGRAM_ID: Pubkey = pubkey!("SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE");

fn read_cargo_program() -> Vec<u8> {
    let mut bin_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bin_path.push("programs/cargo/Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk.bin");
    std::fs::read(bin_path).unwrap()
}

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

    let cargo_program_bytes = read_cargo_program();
    let player_profile_program_bytes = read_player_profile_program();
    let profile_faction_program_bytes = read_profile_faction();
    let sage_program_bytes = read_sage_program();

    svm.airdrop(&funder_pk, 10_000_000_000).unwrap();
    svm.airdrop(&wallet_pk, 10_000_000_000).unwrap();
    svm.add_program(CARGO_PROGRAM_ID, &cargo_program_bytes);
    svm.add_program(PLAYER_PROFILE_PROGRAM_ID, &player_profile_program_bytes);
    svm.add_program(PROFILE_FACTION_PROGRAM_ID, &profile_faction_program_bytes);
    svm.add_program(SAGE_PROGRAM_ID, &sage_program_bytes);

    let authority_kp = Keypair::new();
    let authority_pk = authority_kp.pubkey();

    let sage_kp = Keypair::new();
    let sage_pk = sage_kp.pubkey();

    let player_profile_kp = Keypair::new();
    let player_profile_pk = player_profile_kp.pubkey();

    let player_kp = Keypair::new();
    let player_pk = player_kp.pubkey();

    // TODO: create a "super" user profile
    // // Cargo Superuser // 1
    // {
    //     key: _cargoSigner.publicKey(),
    //     expireTime: null,
    //     scope: cargoProgram.programId,
    //     permissions: CargoPermissions.all(),
    //   },
    //   // Fleet Superuser // 2
    //   {
    //     key: _sageSigner.publicKey(),
    //     expireTime: null,
    //     scope: program.programId,
    //     permissions: SagePermissions.all(),
    //   },
    //   // SagePermissions::MANAGE_FLEET // 3
    //   {
    //     key: _fleetManagerKey.publicKey(),
    //     expireTime: null,
    //     scope: program.programId,
    //     permissions: SagePermissions.manageFleetPermissions(),
    //   },
    //   // SagePermissions::REMOVE_SHIP_ESCROW // 4
    //   {
    //     key: _removeShipEscrowKey.publicKey(),
    //     expireTime: null,
    //     scope: program.programId,
    //     permissions: SagePermissions.removeShipEscrowPermissions(),
    //   },
    //   // SagePermissions::MANAGE_SHIP // 5
    //   {
    //     key: _shipManagerKey.publicKey(),
    //     expireTime: null,
    //     scope: program.programId,
    //     permissions: SagePermissions.manageShipPermissions(),
    //   },
    //   // SagePermissions::MANAGE_FLEET_CARGO // 6
    //   {
    //     key: _manageFleetCargoKey.publicKey(),
    //     expireTime: null,
    //     scope: program.programId,
    //     permissions: SagePermissions.manageFleetCargoPermissions(),
    //   },
    let create_profile_ix = Instruction {
        program_id: PLAYER_PROFILE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(wallet_pk, true), // funder (writable, signer)
            AccountMeta::new(player_profile_pk, true), // profile (writable, signer)
            AccountMeta::new_readonly(system_program::ID, false), // system program
            AccountMeta::new(authority_pk, true), // auth authority (writable, signer)
            AccountMeta::new(authority_pk, true), // cargo manager (writable, signer)
            AccountMeta::new(authority_pk, true), // sage manager (writable, signer)
        ],
        data: CreateProfile {
            _key_permissions: vec![
                // Auth Authority - Full permissions
                AddKeyInput {
                    scope: AnchorPubkey::new_from_array(PLAYER_PROFILE_PROGRAM_ID.to_bytes()),
                    expire_time: -1,
                    permissions: [0xFF, 0xFF, 0, 0, 0, 0, 0, 0], // All permissions enabled
                },
                // Cargo Superuser
                AddKeyInput {
                    scope: AnchorPubkey::new_from_array(CARGO_PROGRAM_ID.to_bytes()),
                    expire_time: -1,
                    permissions: [0xFF, 0xFF, 0, 0, 0, 0, 0, 0], // CargoPermissions.all()
                },
                // Fleet (Sage) Superuser
                AddKeyInput {
                    scope: AnchorPubkey::new_from_array(SAGE_PROGRAM_ID.to_bytes()),
                    expire_time: -1,
                    // byte[0] = 0xFF: MANAGE_GAME through MANAGE_MINE_ITEM
                    // byte[1] = 0xFF: MANAGE_RESOURCE through MANAGE_CARGO_POD
                    // byte[2] = 0xFF: ADD_REMOVE_CARGO through SCAN_SURVEY_DATA_UNIT
                    // byte[3] = 0x1F: DO_STAR_BASE_UPKEEP through WITHDRAW_CREW (only 5 bits used)
                    // bytes[4-7] = 0x00: Unused bytes in the 8-byte array
                    permissions: [
                        0xFF, // byte[0]: All first 8 permissions (bits 0-7) = 11111111
                        0xFF, // byte[1]: All next 8 permissions (bits 0-7) = 11111111
                        0xFF, // byte[2]: All next 8 permissions (bits 0-7) = 11111111
                        0x1F, // byte[3]: Only first 5 permissions (bits 0-4) = 00011111
                        0x00, // byte[4]: Unused
                        0x00, // byte[5]: Unused
                        0x00, // byte[6]: Unused
                        0x00, // byte[7]: Unused
                    ], // SagePermissions.all()
                },
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

    let (player_faction_pda, _bump) = Pubkey::find_program_address(
        &[b"player_faction", player_profile_pk.as_ref()],
        &PROFILE_FACTION_PROGRAM_ID,
    );

    let choose_faction_ix = Instruction {
        program_id: PROFILE_FACTION_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(authority_pk, true), // pub key: Signer<'info>,
            AccountMeta::new(wallet_pk, true),    // pub funder: Signer<'info>,
            AccountMeta::new(player_profile_pk, false), // pub profile: AccountInfo<'info>,
            AccountMeta::new(player_faction_pda, false), // pub faction: AccountInfo<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // system program
        ],
        data: ChooseFaction {
            _key_index: 0,
            _faction: Faction::Ustur,
        }
        .data(),
    };

    let message = Message::new(&[choose_faction_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&authority_kp, &wallet_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // create cargo stats definition
    let cargo_stats_definition_kp = Keypair::new();
    let cargo_stats_definition_pk = cargo_stats_definition_kp.pubkey();
    let cargo_kp = Keypair::new();
    let cargo_pk = cargo_kp.pubkey();

    let init_definition_ix = Instruction {
        program_id: CARGO_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(player_profile_pk, false), // pub profile: AccountInfo<'info>,
            AccountMeta::new(wallet_pk, true),          // pub funder: Signer<'info>,
            AccountMeta::new(cargo_stats_definition_pk, true), // pub stats_definition: Signer<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
        ],
        data: InitDefinition {
            _input: InitDefinitionInput { cargo_stats: 1 },
        }
        .data(),
    };

    let message = Message::new(&[init_definition_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&wallet_kp, &cargo_stats_definition_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // init game
    let game_kp = Keypair::new();
    let game_pk = game_kp.pubkey();

    let init_game_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(authority_pk, true), // pub signer: Signer<'info>,
            AccountMeta::new_readonly(player_profile_pk, false), // pub profile: AccountInfo<'info>,
            AccountMeta::new(wallet_pk, true),    // pub funder: Signer<'info>,
            AccountMeta::new(game_pk, true),      // pub game_id: Signer<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
        ],
        data: InitGame {}.data(),
    };

    let message = Message::new(&[init_game_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&authority_kp, &wallet_kp, &game_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    let game_acc = svm.get_account(&game_pk).unwrap();
    let game_data = Game::try_deserialize(&mut &game_acc.data[..]).unwrap();

    let (game_state_pda, _bump) = Pubkey::find_program_address(
        &[
            b"GameState",
            game_pk.as_ref(),
            &(game_data.update_id + 1).to_le_bytes(),
        ],
        &SAGE_PROGRAM_ID,
    );

    // init game state
    let init_game_state_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(authority_pk, true), // InitGameStateGameAndProfile<'info> pub key: Signer<'info>,
            AccountMeta::new_readonly(player_profile_pk, false), // InitGameStateGameAndProfile<'info>pub profile: AccountInfo<'info>,
            AccountMeta::new_readonly(game_pk, false), // InitGameStateGameAndProfile<'info> pub game_id: AccountInfo<'info>,
            AccountMeta::new(wallet_pk, true), // pub funder: Signer<'info>,
            AccountMeta::new(game_state_pda, false), // pub game_state: AccountInfo<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
        ],
        data: InitGameState {
            _input: InitGameStateInput {
                key_index: 2, // SAGE_MANAGER
            },
        }
        .data(),
    };

    let message = Message::new(&[init_game_state_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&authority_kp, &wallet_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // update game state
    let update_game_state_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(authority_pk, true), // UpdateGameStateGameAndProfile<'info> pub key: Signer<'info>,
            AccountMeta::new_readonly(player_profile_pk, false), // UpdateGameStateGameAndProfile<'info>pub profile: AccountInfo<'info>,
            AccountMeta::new_readonly(game_pk, false), // UpdateGameStateGameAndProfile<'info> pub game_id: AccountInfo<'info>,
            AccountMeta::new(game_state_pda, false), // pub game_state: AccountInfo<'info>,
            AccountMeta::new(Pubkey::default(), false), // old_recipe_for_upgrade
            AccountMeta::new(Pubkey::default(), false), // new_recipe_for_upgrade
            AccountMeta::new(Pubkey::default(), false), // recipe_category_for_level
        ],
        data: UpdateGameState {
            _input: UpdateGameStateInput {
                fleet: Some(FleetInput {
                    starbase_level_info_array: Some(vec![StarbaseLevelInfoArrayInput {
                        level: 6,
                        faction: Faction::Ustur as u8,
                        hp: 6000,
                        sp: 6000,
                        sector_ring_available: SectorRing::Inner,
                        warp_lane_movement_fee: 0,
                    }]),
                    upkeep_info_array: None,
                    max_fleet_size: Some(64),
                }),
                misc: None,
                key_index: 2, // SAGE_MANAGER
            },
        }
        .data(),
    };

    let message = Message::new(&[update_game_state_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&authority_kp, &wallet_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // update game
    // let ammo_kp = Keypair::new();
    // let ammo_pk = ammo_kp.pubkey();

    // let food_kp = Keypair::new();
    // let food_pk = food_kp.pubkey();

    // let fuel_kp = Keypair::new();
    // let fuel_pk = fuel_kp.pubkey();

    // let repair_kit_kp = Keypair::new();
    // let repair_kit_pk = repair_kit_kp.pubkey();

    // // ammo = 1 << 2      // 0b00000100
    // // food = 1 << 3      // 0b00001000
    // // fuel = 1 << 4      // 0b00010000
    // // repairKit = 1 << 5 // 0b00100000
    // let mints: u8 = (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5); // 0b00111100 = 60 in decimal

    let update_game_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(authority_pk, true), // UpdateGameGameAndProfile<'info> pub key: Signer<'info>,
            AccountMeta::new_readonly(player_profile_pk, false), // UpdateGameGameAndProfile<'info>pub profile: AccountInfo<'info>,
            AccountMeta::new(game_pk, false), // UpdateGameGameAndProfile<'info> pub game_id: AccountInfo<'info>,
            // // Add the mint accounts as remaining_accounts
            // AccountMeta::new_readonly(ammo_pk, false),
            // AccountMeta::new_readonly(food_pk, false),
            // AccountMeta::new_readonly(fuel_pk, false),
            // AccountMeta::new_readonly(repair_kit_pk, false),
            AccountMeta::new_readonly(cargo_stats_definition_pk, false),
        ],
        data: UpdateGame {
            _input: UpdateGameInput {
                cargo: 1,
                crafting: 0,
                mints: 0,
                vaults: 0,
                points: 0,
                risk_zones: None, // pub risk_zones: Option<RiskZonesDataUnpacked>,
                key_index: 2,     // SAGE_MANAGER
            },
        }
        .data(),
    };

    let message = Message::new(&[update_game_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&authority_kp, &wallet_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // activate game state
    let activate_game_state_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(authority_pk, true), // ActivateGameStateGameAndProfile<'info> pub key: Signer<'info>,
            AccountMeta::new_readonly(player_profile_pk, false), // ActivateGameStateGameAndProfile<'info>pub profile: AccountInfo<'info>,
            AccountMeta::new(game_pk, false), // ActivateGameStateGameAndProfile<'info> pub game_id: AccountInfo<'info>,
            AccountMeta::new(game_state_pda, false), // pub game_state: AccountInfo<'info>,
        ],
        data: ActivateGameState {
            _input: ManageGameInput {
                key_index: 2, // SAGE_MANAGER
            },
        }
        .data(),
    };

    let message = Message::new(&[activate_game_state_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&authority_kp, &wallet_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    let game_state_acc = svm.get_account(&game_state_pda).unwrap();
    let game_state_data = GameState::try_deserialize(&mut &game_state_acc.data[..]).unwrap();
    assert_eq!(game_pk.as_ref(), game_state_data.game_id.as_ref());

    // register sector
    let (sector_pda, _bump) = Pubkey::find_program_address(
        &[
            b"Sector",
            game_pk.as_ref(),
            &(1_i64).to_le_bytes(),
            &(1_i64).to_le_bytes(),
        ],
        &SAGE_PROGRAM_ID,
    );

    let register_sector_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(authority_pk, true), // RegisterSectorGameAndProfile<'info> pub key: Signer<'info>,
            AccountMeta::new_readonly(player_profile_pk, false), // RegisterSectorGameAndProfile<'info>pub profile: AccountInfo<'info>,
            AccountMeta::new_readonly(game_pk, false), // RegisterSectorGameAndProfile<'info> pub game_id: AccountInfo<'info>,
            AccountMeta::new(wallet_pk, true),         // pub funder: Signer<'info>,
            AccountMeta::new_readonly(player_profile_pk, false), // pub discoverer: AccountInfo<'info>,
            AccountMeta::new(sector_pda, false),                 // pub sector: AccountInfo<'info>
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
        ],
        data: RegisterSector {
            _coordinates: [1, 1],
            _name: {
                let mut name = [0u8; 64];
                let sector_name = b"Super Sector";
                name[..sector_name.len()].copy_from_slice(sector_name);
                name
            },
            _key_index: 2, // SAGE_MANAGER
        }
        .data(),
    };

    let message = Message::new(&[register_sector_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&authority_kp, &wallet_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // TODO: setup crew (fleetCRUD.test.ts 868)

    // create starbase
    let sector_acc = svm.get_account(&sector_pda).unwrap();
    let sector_data = Sector::try_deserialize(&mut &sector_acc.data[..]).unwrap();
    assert_eq!(game_pk.as_ref(), sector_data.game_id.as_ref());

    let (starbase_pda, _bump) = Pubkey::find_program_address(
        &[
            b"Starbase",
            game_pk.as_ref(),
            &sector_data.coordinates[0].to_le_bytes(),
            &sector_data.coordinates[1].to_le_bytes(),
        ],
        &SAGE_PROGRAM_ID,
    );

    let register_starbase_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(wallet_pk, true),     // pub funder: Signer<'info>,
            AccountMeta::new(starbase_pda, false), // pub starbase: AccountInfo<'info>,
            AccountMeta::new_readonly(sector_pda, false), // pub sector: AccountInfo<'info>
            AccountMeta::new_readonly(authority_pk, true), // RegisterStarbaseGameStateAndProfileGameAndProfile<'info> pub key: Signer<'info>,
            AccountMeta::new_readonly(player_profile_pk, false), // RegisterStarbaseGameStateAndProfileGameAndProfile<'info> pub profile: AccountInfo<'info>,
            AccountMeta::new_readonly(game_pk, false), // RegisterStarbaseGameStateAndProfileGameAndProfile<'info> pub game_id: AccountInfo<'info>,
            AccountMeta::new_readonly(game_state_pda, false), // pub game_state: AccountInfo<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
        ],
        data: RegisterStarbase {
            _input: RegisterStarbaseInputUnpacked {
                name: {
                    let mut name = [0u8; 64];
                    let starbase_name = b"Starbase Alpha";
                    name[..starbase_name.len()].copy_from_slice(starbase_name);
                    name
                },
                sub_coordinates: [1, 1],
                starbase_level_index: 6,
                faction: Faction::Ustur as u8,
                key_index: 2, // SAGE_MANAGER
            },
        }
        .data(),
    };

    let message = Message::new(&[register_starbase_ix], Some(&wallet_pk));
    let tx = Transaction::new(
        &[&wallet_kp, &authority_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // create sage player profile
    let (sage_player_profile_pda, _bump) = Pubkey::find_program_address(
        &[
            b"sage_player_profile",
            player_profile_pk.as_ref(),
            game_pk.as_ref(),
        ],
        &SAGE_PROGRAM_ID,
    );

    let register_sage_player_profile_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(player_profile_pk, false), // pub profile: AccountInfo<'info>,
            AccountMeta::new(wallet_pk, true),                   // pub funder: Signer<'info>,
            AccountMeta::new(sage_player_profile_pda, false), // pub sage_player_profile: AccountInfo<'info>,
            AccountMeta::new_readonly(game_pk, false), // RegisterSagePlayerProfileGameAccounts<'info> pub game_id: AccountInfo<'info>,
            AccountMeta::new_readonly(game_state_pda, false), // RegisterSagePlayerProfileGameAccounts<'info> pub game_state: AccountInfo<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
        ],
        data: RegisterSagePlayerProfile {}.data(),
    };

    let message = Message::new(&[register_sage_player_profile_ix], Some(&wallet_pk));
    let tx = Transaction::new(&[&wallet_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // create starbase player
    let starbase_seq_id: u16 = 0;
    let (starbase_player_pda, _bump) = Pubkey::find_program_address(
        &[
            b"starbase_player",
            starbase_pda.as_ref(),
            sage_player_profile_pda.as_ref(),
            &starbase_seq_id.to_le_bytes(),
        ],
        &SAGE_PROGRAM_ID,
    );

    let register_starbase_player_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(wallet_pk, true), // pub funder: Signer<'info>,
            AccountMeta::new_readonly(game_pk, false), // RegisterStarbasePlayerGameAccounts<'info> pub game_id: AccountInfo<'info>,
            AccountMeta::new_readonly(game_state_pda, false), // RegisterStarbasePlayerGameAccounts<'info> pub game_state: AccountInfo<'info>,
            AccountMeta::new_readonly(sage_player_profile_pda, false), // pub sage_player_profile: AccountInfo<'info>,
            AccountMeta::new_readonly(player_faction_pda, false), // pub profile_faction: AccountInfo<'info>,
            AccountMeta::new_readonly(starbase_pda, false), // pub starbase: AccountInfo<'info>,
            AccountMeta::new(starbase_player_pda, false), // pub starbase_player: AccountInfo<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
        ],
        data: RegisterStarbasePlayer {}.data(),
    };

    let message = Message::new(&[register_starbase_player_ix], Some(&wallet_pk));
    let tx = Transaction::new(&[&wallet_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // TODO: _createShipMint()
    let mint_ship_kp = Keypair::new();
    // FIXME: issues with `litesvm-token`

    // export function createMint(
    //     mint: AsyncSigner,
    //     decimals: number,
    //     mintAuthority: PublicKey,
    //     freezeAuthority: PublicKey | null
    //   ): InstructionReturn {
    //     // eslint-disable-next-line require-await
    //     return async (funder) => [
    //       {
    //         instruction: SystemProgram.createAccount({
    //           fromPubkey: funder.publicKey(),
    //           lamports: calculateMinimumRent(MINT_SIZE),
    //           newAccountPubkey: mint.publicKey(),
    //           programId: TOKEN_PROGRAM_ID,
    //           space: MINT_SIZE,
    //         }),
    //         signers: [mint, funder],
    //       },
    //       {
    //         instruction: createInitializeMintInstruction(
    //           mint.publicKey(),
    //           decimals,
    //           mintAuthority,
    //           freezeAuthority
    //         ),
    //         signers: [],
    //       },
    //     ];
    //   }

    // TODO: mintAndImportCrewToGame()

    assert!(true);
}
