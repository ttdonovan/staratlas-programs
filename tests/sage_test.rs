use anchor_lang::{prelude::Pubkey as AnchorPubkey, AccountDeserialize, InstructionData};
use litesvm::LiteSVM;
use solana_sdk::{
    account::Account,
    feature_set::FeatureSet,
    instruction::{AccountMeta, Instruction},
    message::Message,
    program_option::COption,
    program_pack::Pack,
    pubkey,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use spl_associated_token_account_client::address::get_associated_token_address;
use spl_token::{
    state::{Account as TokenAccount, AccountState},
    ID as TOKEN_PROGRAM_ID,
};

use staratlas_cargo::{instruction::InitDefinition, typedefs::InitDefinitionInput};
use staratlas_crew::state::CrewConfig;
use staratlas_player_profile::{instruction::CreateProfile, typedefs::AddKeyInput};
use staratlas_profile_faction::{instruction::ChooseFaction, typedefs::Faction};
use staratlas_sage::{
    instruction::{RegisterSagePlayerProfile, RegisterStarbasePlayer, UpdateGame, UpdateGameState},
    state::{Game, GameState, Sector},
    typedefs::{
        FleetInput, SectorRing, StarbaseLevelInfoArrayInput, UpdateGameInput, UpdateGameStateInput,
    },
};

use staratlas_starbased_sdk as based_sdk;

mod shared;
use shared::{
    constants::{
        CARGO_PROGRAM_BYTES, CARGO_PROGRAM_ID, CREW_PROGRAM_BYTES, CREW_PROGRAM_ID,
        PLAYER_PROFILE_PROGRAM_BYTES, PLAYER_PROFILE_PROGRAM_ID, PROFILE_FACTION_PROGRAM_BYTES,
        PROFILE_FACTION_PROGRAM_ID, SAGE_PROGRAM_BYTES, SAGE_PROGRAM_ID,
    },
    helpers,
};

#[test]
fn sage_test() {
    let feature_set = FeatureSet::all_enabled();
    let mut svm = LiteSVM::default()
        .with_feature_set(feature_set)
        .with_builtins()
        .with_lamports(1_000_000_000_000_000)
        .with_sysvars();

    let funder_kp = Keypair::new();
    let funder_pk = funder_kp.pubkey();

    let wallet_kp = Keypair::new();
    let wallet_pk = wallet_kp.pubkey();

    svm.airdrop(&funder_pk, 10_000_000_000).unwrap();
    svm.airdrop(&wallet_pk, 10_000_000_000).unwrap();
    svm.add_program(CARGO_PROGRAM_ID, CARGO_PROGRAM_BYTES);
    svm.add_program(CREW_PROGRAM_ID, CREW_PROGRAM_BYTES);
    svm.add_program(PLAYER_PROFILE_PROGRAM_ID, PLAYER_PROFILE_PROGRAM_BYTES);
    svm.add_program(PROFILE_FACTION_PROGRAM_ID, PROFILE_FACTION_PROGRAM_BYTES);
    svm.add_program(SAGE_PROGRAM_ID, SAGE_PROGRAM_BYTES);

    // starbased-sdk: profile create profile (player)
    let player_profile_kp = Keypair::new();
    let player_profile_pk = based_sdk::profile::CreateProfile::new(&player_profile_kp, &wallet_kp)
        .send(&mut svm)
        .unwrap();
    dbg!(&player_profile_pk);

    let authority_kp = Keypair::new();
    let authority_pk = authority_kp.pubkey();

    let sage_profile_kp = Keypair::new();
    let sage_profile_pk = sage_profile_kp.pubkey();

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
            AccountMeta::new(sage_profile_pk, true), // profile (writable, signer)
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
        &[&wallet_kp, &sage_profile_kp, &authority_kp],
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
            AccountMeta::new(player_profile_pk, true), // pub key: Signer<'info>,
            AccountMeta::new(wallet_pk, true),         // pub funder: Signer<'info>,
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
        &[&player_profile_kp, &wallet_kp],
        message,
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    // create cargo stats definition
    let cargo_stats_definition_kp = Keypair::new();
    let cargo_stats_definition_pk = cargo_stats_definition_kp.pubkey();

    let init_definition_ix = Instruction {
        program_id: CARGO_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(sage_profile_pk, false), // pub profile: AccountInfo<'info>,
            AccountMeta::new(wallet_pk, true),        // pub funder: Signer<'info>,
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

    // starbased-sdk: admin create game
    let game_kp = Keypair::new();
    let game_pk = based_sdk::admin::CreateGame::new(&authority_kp, &sage_profile_pk, &wallet_kp)
        .set_game_kp(game_kp)
        .send(&mut svm)
        .unwrap();
    dbg!(&game_pk);

    let game_acc = svm.get_account(&game_pk).unwrap();
    let game_data = Game::try_deserialize(&mut &game_acc.data[..]).unwrap();

    // starbased-sdk: admin create game state
    let game_state_pk = based_sdk::admin::CreateGameState::new(
        &authority_kp,
        &sage_profile_pk,
        &game_pk,
        &wallet_kp,
    )
    .set_game_update_id(game_data.update_id)
    .set_profile_key_index(2) // SAGE_MANAGER
    .send(&mut svm)
    .unwrap();
    dbg!(&game_state_pk);

    // update game state
    let update_game_state_ix = Instruction {
        program_id: SAGE_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(authority_pk, true), // UpdateGameStateGameAndProfile<'info> pub key: Signer<'info>,
            AccountMeta::new_readonly(sage_profile_pk, false), // UpdateGameStateGameAndProfile<'info>pub profile: AccountInfo<'info>,
            AccountMeta::new_readonly(game_pk, false), // UpdateGameStateGameAndProfile<'info> pub game_id: AccountInfo<'info>,
            AccountMeta::new(game_state_pk, false),    // pub game_state: AccountInfo<'info>,
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
            AccountMeta::new_readonly(sage_profile_pk, false), // UpdateGameGameAndProfile<'info>pub profile: AccountInfo<'info>,
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

    // starbased-sdk: admin activate game state
    let _ = based_sdk::admin::ActivateGameState::new(
        &authority_kp,
        &sage_profile_pk,
        &game_pk,
        &game_state_pk,
        &wallet_kp,
    )
    .set_profile_key_index(2) // SAGE_MANAGER
    .send(&mut svm)
    .unwrap();

    let game_state_acc = svm.get_account(&game_state_pk).unwrap();
    let game_state_data = GameState::try_deserialize(&mut &game_state_acc.data[..]).unwrap();
    assert_eq!(game_pk.as_ref(), game_state_data.game_id.as_ref());

    // starbased-sdk: admin register sector
    let sector_pk = based_sdk::admin::RegisterSector::new(
        &authority_kp,
        &sage_profile_pk,
        &sage_profile_pk, // discoverer
        &game_pk,
        &wallet_kp,
    )
    .set_coordinates([1, 1])
    .set_name("Super Sector".into())
    .set_profile_key_index(2) // SAGE_MANAGER
    .send(&mut svm)
    .unwrap();
    dbg!(sector_pk);

    // TODO: setup crew (fleetCRUD.test.ts 868)

    // create starbase
    let sector_acc = svm.get_account(&sector_pk).unwrap();
    let sector_data = Sector::try_deserialize(&mut &sector_acc.data[..]).unwrap();
    assert_eq!(game_pk.as_ref(), sector_data.game_id.as_ref());

    // starbased-sdk: admin register starbase
    let starbase_pk = based_sdk::admin::RegisterStarbase::new(
        &authority_kp,
        &sage_profile_pk,
        &game_pk,
        &game_state_pk,
        &sector_pk,
        &wallet_kp,
    )
    .set_coordinates([1, 1])
    .set_name("Starbase Alpha".into())
    .set_sub_coordinates([1, 1])
    .set_starbase_level_index(6)
    .set_faction(Faction::Ustur as u8)
    .set_profile_key_index(2) // SAGE_MANAGER
    .send(&mut svm)
    .unwrap();
    dbg!(starbase_pk);

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
            AccountMeta::new_readonly(game_state_pk, false), // RegisterSagePlayerProfileGameAccounts<'info> pub game_state: AccountInfo<'info>,
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
            starbase_pk.as_ref(),
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
            AccountMeta::new_readonly(game_state_pk, false), // RegisterStarbasePlayerGameAccounts<'info> pub game_state: AccountInfo<'info>,
            AccountMeta::new_readonly(sage_player_profile_pda, false), // pub sage_player_profile: AccountInfo<'info>,
            AccountMeta::new_readonly(player_faction_pda, false), // pub profile_faction: AccountInfo<'info>,
            AccountMeta::new_readonly(starbase_pk, false), // pub starbase: AccountInfo<'info>,
            AccountMeta::new(starbase_player_pda, false), // pub starbase_player: AccountInfo<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
        ],
        data: RegisterStarbasePlayer {}.data(),
    };

    let message = Message::new(&[register_starbase_player_ix], Some(&wallet_pk));
    let tx = Transaction::new(&[&wallet_kp], message, svm.latest_blockhash());
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    let (crew_config_ix, crew_merkle_tree_pk) =
        helpers::setup_crew_config_instructions(&sage_profile_pk, &wallet_pk, &game_pk);

    let tx = Transaction::new_signed_with_payer(
        &[crew_config_ix],
        Some(&wallet_pk),
        &[&wallet_kp],
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    let (crew_config_pda, _bump) =
        Pubkey::find_program_address(&[b"crew_config", game_pk.as_ref()], &CREW_PROGRAM_ID);

    let crew_config_acc = svm.get_account(&crew_config_pda).unwrap();
    let crew_config_data = CrewConfig::try_deserialize(&mut &crew_config_acc.data[..]).unwrap();
    assert_eq!(crew_config_data.seed_pubkey.as_ref(), game_pk.as_ref());
    dbg!(&crew_merkle_tree_pk);

    // see `starbaseCrafting.test.ts` for examples

    // // TODO: import crew to game
    // await mintAndImportCrewToGame(
    //     crewConfigResult.merkleTree,
    //     authority,
    //     crewConfigResult.crewProgramConfig,
    //     await setupUmi(walletSigner, provider.connection),
    //     program,
    //     playerProfile.publicKey(),
    //     ProfileFactionAccount.findAddress(
    //       profileFactionProgram,
    //       playerProfile.publicKey(),
    //     )[0],
    //     _starbasePlayerKey,
    //     _starbaseKey,
    //     _game.publicKey(),
    //     funder,
    //     connection,
    //     [], // knownLeaves
    //     5, // numCrew
    //   );

    // export const mintAndImportCrewToGame = async (
    //     crewMerkleTree: PublicKey,
    //     crewOwner: AsyncSigner,
    //     crewProgramConfig: PublicKey,
    //     umi: Umi,
    //     sageProgram: SageIDLProgram,
    //     profile: PublicKey,
    //     profileFaction: PublicKey,
    //     starbasePlayer: PublicKey,
    //     starbase: PublicKey,
    //     gameId: PublicKey,
    //     funder: AsyncSigner,
    //     connection: Connection,
    //     knownLeaves: UmiPublicKey[] = [],
    //     numCrew = 15,
    //     startingLeafIndex?: number,
    //     treeCreatorOrDelegate?: AsyncSigner,
    //   ) => {
    //     const crewChunks = divideNumber(numCrew, 5);

    //     for (let index = 0; index < crewChunks.length; index++) {
    //       const crewChunk = crewChunks[index];
    //       const mintPrepareResult = await mintAndPrepareCrew(
    //         crewMerkleTree,
    //         crewOwner.publicKey(),
    //         umi,
    //         knownLeaves,
    //         crewChunk,
    //         startingLeafIndex ?? knownLeaves.length,
    //         treeCreatorOrDelegate || crewOwner,
    //       );
    //       const lookupTable = await createNewLookupTable(
    //         removeDuplicateKeys([
    //           sageProgram.programId,
    //           profile,
    //           profileFaction,
    //           crewOwner.publicKey(),
    //           starbasePlayer,
    //           starbase,
    //           crewMerkleTree,
    //           crewProgramConfig,
    //           gameId,
    //           funder.publicKey(),
    //           SagePlayerProfile.findAddress(sageProgram, profile, gameId)[0],
    //           SageCrewConfig.findAddress(sageProgram, gameId)[0],
    //           PublicKey.findProgramAddressSync(
    //             [crewMerkleTree.toBuffer()],
    //             toWeb3JsPublicKey(MPL_BUBBLEGUM_PROGRAM_ID),
    //           )[0],
    //           toWeb3JsPublicKey(SPL_ACCOUNT_COMPRESSION_PROGRAM_ID),
    //           toWeb3JsPublicKey(MPL_BUBBLEGUM_PROGRAM_ID),
    //           toWeb3JsPublicKey(SPL_NOOP_PROGRAM_ID),
    //           SystemProgram.programId,
    //           normalizePublicKey(mintPrepareResult.items[0].creatorHash),
    //           ...mintPrepareResult.items.map((it) => it.proof).flat(),
    //         ]),
    //         funder,
    //         funder,
    //         connection,
    //         undefined, // recentSlot
    //         true, // awaitNewSlot
    //       );

    //       await sendManyToChain(
    //         [
    //           ixToIxReturn(
    //             ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 426 }),
    //           ),
    //           ixToIxReturn(
    //             ComputeBudgetProgram.setComputeUnitLimit({ units: 1_400_000 }),
    //           ),
    //           SagePlayerProfile.addCrewToGame(
    //             sageProgram,
    //             profile,
    //             profileFaction,
    //             crewOwner,
    //             starbasePlayer,
    //             starbase,
    //             crewProgramConfig,
    //             gameId,
    //             {
    //               items: [...mintPrepareResult.items],
    //             },
    //           ),
    //         ],
    //         funder,
    //         connection,
    //         'confirmed',
    //         true,
    //         [lookupTable],
    //       );

    //       const leavesUpdatedAfterAdd = mintPrepareResult.items.map((it) =>
    //         hashLeaf(umi, {
    //           merkleTree: fromWeb3JsPublicKey(crewMerkleTree),
    //           owner: fromWeb3JsPublicKey(
    //             SagePlayerProfile.findAddress(sageProgram, profile, gameId)[0],
    //           ),
    //           leafIndex: it.leafIndex,
    //           metadata: it.metadata,
    //         }),
    //       );
    //       knownLeaves.push(...leavesUpdatedAfterAdd.map((it) => getUmiPublicKey(it)));
    //     }
    //   };

    // // Add to tests/shared/helpers.rs
    // pub fn mock_import_crew_to_game(
    //     svm: &mut LiteSVM,
    //     crew_merkle_tree_pk: &Pubkey,
    //     crew_program_config_pk: &Pubkey,
    //     sage_program_id: &Pubkey,
    //     player_profile_pk: &Pubkey,
    //     player_faction_pda: &Pubkey,
    //     starbase_player_pda: &Pubkey,
    //     starbase_pda: &Pubkey,
    //     game_pk: &Pubkey,
    //     wallet_pk: &Pubkey,
    //     wallet_kp: &Keypair,
    //     authority_pk: &Pubkey,
    //     authority_kp: &Keypair,
    //     num_crew: usize,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     use solana_program::hash::hash;

    //     // Calculate sage player profile address
    //     let (sage_player_profile_pda, _bump) = Pubkey::find_program_address(
    //         &[
    //             b"sage_player_profile",
    //             player_profile_pk.as_ref(),
    //             game_pk.as_ref(),
    //         ],
    //         sage_program_id,
    //     );

    //     // Calculate sage crew config address
    //     let (sage_crew_config_pda, _bump) = Pubkey::find_program_address(
    //         &[
    //             b"sage_crew_config",
    //             game_pk.as_ref(),
    //         ],
    //         sage_program_id,
    //     );

    //     // Create fake merkle tree adapter account
    //     let (merkle_tree_adapter_pda, _bump) = Pubkey::find_program_address(
    //         &[crew_merkle_tree_pk.as_ref()],
    //         &pubkey!("BGUMzZr2wWfD2yzrXFEWTK2HbdYhqQCP2EZoPEkZBD6w"), // Bubblegum program ID
    //     );

    //     // Mock items that would be generated from mintAndPrepareCrew
    //     let mut mock_items = Vec::with_capacity(num_crew);
    //     for i in 0..num_crew {
    //         // Create fake proof data based on index
    //         let leaf_hash = hash(&format!("crew-{}", i).as_bytes()).to_bytes();
    //         let creator_hash = hash(&format!("creator-{}", i).as_bytes()).to_bytes();

    //         // Create fake proof path - in real implementation would be computed from merkle tree
    //         let mut proof = Vec::new();
    //         for j in 0..10 {  // Mock 10 proof elements
    //             proof.push(hash(&format!("proof-{}-{}", i, j).as_bytes()).to_bytes());
    //         }

    //         mock_items.push((leaf_hash, creator_hash, proof, i as u32));
    //     }

    //     // Process in chunks of 5 (like the TypeScript code)
    //     let chunk_size = 5;
    //     for chunk_start in (0..num_crew).step_by(chunk_size) {
    //         let chunk_end = std::cmp::min(chunk_start + chunk_size, num_crew);
    //         let current_chunk = &mock_items[chunk_start..chunk_end];

    //         // Create the add crew to game instruction
    //         let mut accounts = vec![
    //             AccountMeta::new(*wallet_pk, true),                       // funder
    //             AccountMeta::new_readonly(*player_profile_pk, false),      // profile
    //             AccountMeta::new_readonly(*player_faction_pda, false),     // profile faction
    //             AccountMeta::new_readonly(*authority_pk, true),           // crew owner
    //             AccountMeta::new_readonly(*starbase_player_pda, false),    // starbase player
    //             AccountMeta::new_readonly(*starbase_pda, false),           // starbase
    //             AccountMeta::new_readonly(*crew_merkle_tree_pk, false),    // crew merkle tree
    //             AccountMeta::new_readonly(*crew_program_config_pk, false), // crew program config
    //             AccountMeta::new_readonly(*game_pk, false),                // game id
    //             AccountMeta::new(sage_player_profile_pda, false),        // sage player profile
    //             AccountMeta::new_readonly(sage_crew_config_pda, false),   // sage crew config
    //             AccountMeta::new_readonly(merkle_tree_adapter_pda, false), // merkle tree adapter
    //             AccountMeta::new_readonly(pubkey!("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK"), false), // Account compression program
    //             AccountMeta::new_readonly(pubkey!("BGUMzZr2wWfD2yzrXFEWTK2HbdYhqQCP2EZoPEkZBD6w"), false), // Bubblegum program
    //             AccountMeta::new_readonly(pubkey!("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV"), false), // Noop program
    //             AccountMeta::new_readonly(system_program::ID, false),     // System program
    //         ];

    //         // Add creator hash account
    //         accounts.push(AccountMeta::new_readonly(
    //             Pubkey::new_from_array(current_chunk[0].1),  // Use creator hash as pubkey
    //             false,
    //         ));

    //         // Add proof accounts for each item in chunk
    //         for (_, _, proof, _) in current_chunk {
    //             for &proof_element in proof {
    //                 accounts.push(AccountMeta::new_readonly(
    //                     Pubkey::new_from_array(proof_element),
    //                     false,
    //                 ));
    //             }
    //         }

    //         // Create instruction data
    //         // This is a simplified approximation - in reality you'd need to match the exact
    //         // anchor instruction format with discriminator + serialized args
    //         let mut instruction_data = vec![
    //             0x73, 0x3a, 0x55, 0x55, 0xe3, 0x41, 0xe7, 0x01  // Discriminator for AddCrewToGame
    //         ];

    //         // Add item count
    //         instruction_data.extend_from_slice(&(current_chunk.len() as u32).to_le_bytes());

    //         // Add each item's data
    //         for (leaf_hash, _, _, leaf_index) in current_chunk {
    //             // Add leaf hash
    //             instruction_data.extend_from_slice(leaf_hash);

    //             // Add leaf index
    //             instruction_data.extend_from_slice(&leaf_index.to_le_bytes());

    //             // Add empty metadata (simplified)
    //             instruction_data.extend_from_slice(&[0; 32]);
    //         }

    //         // Create the instruction
    //         let add_crew_ix = Instruction {
    //             program_id: *sage_program_id,
    //             accounts,
    //             data: instruction_data,
    //         };

    //         // Create and send transaction
    //         let message = Message::new(&[add_crew_ix], Some(wallet_pk));
    //         let tx = Transaction::new(
    //             &[wallet_kp, authority_kp],
    //             message,
    //             svm.latest_blockhash(),
    //         );

    //         let tx_result = svm.send_transaction(tx);
    //         if let Err(e) = tx_result {
    //             println!("Error adding crew chunk: {:?}", e);
    //             // In actual test you may want to fail, but for sake of getting the test running:
    //             // return Err(e.into());

    //             // Alternatively, for testing convenience:
    //             println!("Simulating success despite error");
    //         }
    //     }

    //     Ok(())
    // }

    // // Then in your sage_test.rs test function:
    // helpers::mock_import_crew_to_game(
    //     &mut svm,
    //     &crew_merkle_tree_pk,
    //     &crew_config_pda,
    //     &SAGE_PROGRAM_ID,
    //     &player_profile_pk,
    //     &player_faction_pda,
    //     &starbase_player_pda,
    //     &starbase_pda,
    //     &game_pk,
    //     &wallet_pk,
    //     &wallet_kp,
    //     &authority_pk,
    //     &authority_kp,
    //     5 // Number of crew to add
    // ).unwrap_or_else(|e| {
    //     eprintln!("Warning: Crew import simulation failed: {}", e);
    // });

    // TODO: _createShipMint()
    // FIXME: issues with `litesvm-token 0.5.0` crate
    let ship_mint = pubkey!("AkNbg12E9PatjkiAWJ3tAbM479gtcoA1gi6Joa925WKi"); // Calico Compakt Hero
    let ata = get_associated_token_address(&wallet_pk, &ship_mint);
    let ship_to_own = 5;

    let token_acc = TokenAccount {
        mint: ship_mint,
        owner: wallet_pk,
        amount: ship_to_own,
        delegate: COption::None,
        state: AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };

    let mut token_acc_bytes = [0u8; TokenAccount::LEN];
    TokenAccount::pack(token_acc, &mut token_acc_bytes).unwrap();

    svm.set_account(
        ata,
        Account {
            lamports: 1_000_000_000,
            data: token_acc_bytes.to_vec(),
            owner: TOKEN_PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();

    let raw_account = svm.get_account(&ata).unwrap();
    let token_acc = TokenAccount::unpack(&raw_account.data).unwrap();
    dbg!(&token_acc);
    assert_eq!(token_acc.amount, ship_to_own);

    assert!(true);
}
