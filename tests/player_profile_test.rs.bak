// use anchor_lang::{prelude::Pubkey as AnchorPubkey, AccountDeserialize, InstructionData};
// use litesvm::LiteSVM;
// use solana_sdk::{
//     feature_set::FeatureSet,
//     instruction::{AccountMeta, Instruction},
//     message::Message,
//     pubkey,
//     pubkey::Pubkey,
//     signature::{Keypair, Signer},
//     system_program,
//     transaction::Transaction,
// };

// use staratlas_player_profile::{instruction::CreateProfile, state::Profile, typedefs::AddKeyInput};

// const PLAYER_PROFILE_PROGRAM_ID: Pubkey = pubkey!("pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9");

// fn read_player_profile_program() -> Vec<u8> {
//     let mut bin_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     bin_path.push("programs/player-profile/pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9.bin");
//     std::fs::read(bin_path).unwrap()
// }

// #[test]
// fn player_profile_test() {
//     let feature_set = FeatureSet::all_enabled();
//     let mut svm = LiteSVM::default()
//         .with_builtins(Some(feature_set))
//         .with_lamports(1_000_000_000_000_000)
//         .with_sysvars();

//     let payer_kp = Keypair::new();
//     let payer_pk = payer_kp.pubkey();

//     let player_profile_program_bytes = read_player_profile_program();

//     svm.airdrop(&payer_pk, 10_000_000_000).unwrap();
//     svm.add_program(PLAYER_PROFILE_PROGRAM_ID, &player_profile_program_bytes);

//     // create a player profile
//     let instruction = Instruction {
//         program_id: PLAYER_PROFILE_PROGRAM_ID,
//         accounts: vec![
//             AccountMeta::new(payer_pk, true),  // funder (writable, signer)
//             AccountMeta::new(payer_pk, false), // profile (writable)
//             AccountMeta::new_readonly(system_program::ID, false), // system program
//             AccountMeta::new(payer_pk, false), // key to authorize(?)
//         ],
//         data: CreateProfile {
//             _key_permissions: vec![AddKeyInput {
//                 scope: AnchorPubkey::new_from_array(PLAYER_PROFILE_PROGRAM_ID.to_bytes()),
//                 expire_time: -1,
//                 permissions: {
//                     let mut out = [0u8; 8];
//                     // First byte contains first 8 permissions
//                     out[0] = (1 << 0) |  // auth (true)
//                              (1 << 1) |  // addKeys (true)
//                              (0 << 2) |  // removeKeys (false)
//                              (0 << 3) |  // changeName (false)
//                              (0 << 4) |  // createRole (false)
//                              (0 << 5) |  // removeRole (false)
//                              (0 << 6) |  // setAuthorizer (false)
//                              (0 << 7); // joinRole (false)

//                     // Second byte contains remaining 4 permissions
//                     out[1] = (0 << 0) |  // leaveRole (false)
//                              (0 << 1) |  // toggleAcceptingMembers (false)
//                              (0 << 2) |  // addMember (false)
//                              (0 << 3); // removeMember (false)

//                     out
//                 },
//             }],
//             _key_threshold: 1,
//         }
//         .data(),
//     };

//     let message = Message::new(&[instruction], Some(&payer_pk));
//     let tx = Transaction::new(&[payer_kp], message, svm.latest_blockhash());
//     let tx_result = svm.send_transaction(tx);
//     assert!(tx_result.is_ok());

//     // let tx_meta = tx_result.unwrap();
//     // dbg!(&tx_meta);

//     // let (profile_pda, _bump) = Pubkey::find_program_address(
//     //     &[
//     //         b"profile",
//     //         // ???,
//     //         payer_pk.as_ref(),
//     //     ],
//     //     &PLAYER_PROFILE_PROGRAM_ID,
//     // );
//     // dbg!(&profile_pda);
//     // let profile_acc = svm.get_account(&profile_pda);
//     // dbg!(&profile_acc);
//     // let profile_data = Profile::try_deserialize(&mut &profile_acc.data[..]).unwrap();

//     assert!(true);
// }
