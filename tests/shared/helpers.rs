#![allow(dead_code)]
use anchor_lang::InstructionData;
use litesvm::{LiteSVM, types::FailedTransactionMetadata};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction, system_program,
    transaction::Transaction,
};

use super::constants::CREW_PROGRAM_ID;

// https://github.com/LiteSVM/litesvm/blob/master/crates/token/src/create_mint.rs
pub fn create_mint<'a>(
    svm: &'a mut LiteSVM,
    payer: &'a Keypair,
    mint_kp: &'a Keypair,
    decimals: u8,
    authority_pk: &'a Pubkey,
) -> Result<Pubkey, FailedTransactionMetadata> {
    use spl_token::{ID as TOKEN_PROGRAM_ID, instruction::initialize_mint2, state::Mint};

    let mint_size = Mint::LEN;
    let mint_pk = mint_kp.pubkey();
    let payer_pk = payer.pubkey();

    let ix1 = system_instruction::create_account(
        &payer_pk,
        &mint_pk,
        svm.minimum_balance_for_rent_exemption(mint_size),
        mint_size as u64,
        &TOKEN_PROGRAM_ID,
    );

    let ix2 = initialize_mint2(&TOKEN_PROGRAM_ID, &mint_pk, authority_pk, None, decimals)?;

    let tx = Transaction::new_signed_with_payer(
        &[ix1, ix2],
        Some(&payer_pk),
        &[payer, &mint_kp],
        svm.latest_blockhash(),
    );

    svm.send_transaction(tx)?;

    Ok(mint_pk)
}

pub fn setup_crew_config_instructions(
    profile_pk: &Pubkey,
    funder_pk: &Pubkey,
    game_pk: &Pubkey,
) -> (Instruction, Pubkey) {
    use staratlas_crew::{
        instruction::RegisterCrewConfig,
        typedefs::{CrewCreatorUnpacked, RegisterCrewConfigArgs},
    };

    let crew_collection_kp = Keypair::new();
    let crew_creator_kp = Keypair::new();

    let (crew_config_pda, _bump) =
        Pubkey::find_program_address(&[b"crew_config", game_pk.as_ref()], &CREW_PROGRAM_ID);

    let crew_merkle_tree_kp = Keypair::new();
    let crew_merkle_tree_pk = crew_merkle_tree_kp.pubkey();

    let register_crew_config_ix = Instruction {
        program_id: CREW_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(*profile_pk, false), // pub profile: AccountInfo<'info>,
            AccountMeta::new(*funder_pk, true),            // pub funder: Signer<'info>,
            AccountMeta::new(crew_config_pda, false),      // pub crew_config: AccountInfo<'info>,
            AccountMeta::new_readonly(*game_pk, false),    // pub seed_pubkey: AccountInfo<'info>,
            AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
            AccountMeta::new(crew_merkle_tree_pk, false),
        ],
        data: RegisterCrewConfig {
            _args: RegisterCrewConfigArgs {
                name_prefix: "Test".into(),
                symbol: "Test".into(),
                uri_prefix: "Test".into(),
                seller_fee_basis_points: 20,
                collection: crew_collection_kp.pubkey(),
                creators: vec![CrewCreatorUnpacked {
                    key: crew_creator_kp.pubkey(),
                    share: 100,
                }],
            },
        }
        .data(),
    };

    (register_crew_config_ix, crew_merkle_tree_pk)
}

// const crewConfigResult = await setupCrewConfigInstructions(
//     authority,
//     playerProfile.publicKey(),
//     0,
//     _game.publicKey(),
//     program,
//     crewProgram,
//     await setupUmi(walletSigner, provider.connection),
//     provider.connection,
//   );

// async configureCrew() {
//     const crewConfigResult = await setupCrewConfigInstructions(
//       this.profiles.superuser.key,
//       this.profiles.superuser.profile,
//       this.profiles.superuser.index,
//       this.gameId,
//       this.program,
//       this.crewProgram,
//       await setupUmi(this.funder, this.connection, [
//         this.profiles.superuser.key,
//       ]),
//       this.connection,
//     );
//     const { instructions, ...rest } = crewConfigResult;
//     if (instructions.length > 0) {
//       await this.sendManyToChain(instructions, true);
//     }
//     this.crew = {
//       crewMerkleTree: rest.merkleTree,
//       sageCrewConfig: rest.sageCrewConfig[0],
//       crewProgramConfig: rest.crewProgramConfig,
//       knownLeaves: [],
//     };
//     return rest;
//   }

// export const setupCrewConfigInstructions = async (
//     adminSigner: AsyncSigner,
//     adminProfile: PublicKey,
//     adminKeyIndex: number,
//     gameId: PublicKey,
//     sageProgram: SageIDLProgram,
//     crewProgram: CrewIDLProgram,
//     umi: Umi,
//     connection: Connection,
//     maxDepth = CREW_TREE_MAX_DEPTH,
//     maxBufferSize = CREW_TREE_MAX_BUFFER_SIZE,
//     canopyDepth = CREW_TREE_CANOPY_DEPTH,
//   ) => {
//     if (!adminSigner.inner) {
//       throw 'adminSigner Keypair not found';
//     }
//     const crewCollectionKeypair = Keypair.generate();
//     const crewCreatorKeypair = Keypair.generate();
//     const [crewConfigPubkey, _crewConfigBump] = CrewConfig.findAddress(
//       crewProgram,
//       gameId,
//     );
//     const crewConfigAccount = await readFromRPCNullable(
//       connection,
//       crewProgram,
//       crewConfigPubkey,
//       CrewConfig,
//     );
//     const instructions = [];
//     let merkleTree: PublicKey;
//     if (crewConfigAccount) {
//       // use the last tree in the array to ensure freshness incase it is created in another package's tests
//       merkleTree =
//         crewConfigAccount.merkleTrees[crewConfigAccount.merkleTrees.length - 1];
//     } else {
//       const merkleTreeCreated = await createTree(umi, {
//         public: false,
//         maxDepth,
//         maxBufferSize,
//         canopyDepth,
//         treeCreator: createSignerFromKeypair(
//           umi,
//           fromWeb3JsKeypair(adminSigner.inner() as Keypair),
//         ),
//       });
//       await setTreeDelegate(umi, {
//         merkleTree: merkleTreeCreated,
//         newTreeDelegate: fromWeb3JsPublicKey(crewConfigPubkey),
//       }).sendAndConfirm(umi, {
//         send: { skipPreflight: true },
//       });
//       merkleTree = toWeb3JsPublicKey(merkleTreeCreated);
//       instructions.push(
//         registerCrewConfig(
//           crewProgram,
//           adminProfile,
//           {
//             namePrefix: 'Test',
//             symbol: 'Test',
//             uriPrefix: 'Test',
//             sellerFeeBasisPoints: 20,
//             collection: crewCollectionKeypair.publicKey,
//             creators: [{ key: crewCreatorKeypair.publicKey, share: 100 }],
//           },
//           [merkleTree],
//           gameId, // seedPubkey
//         ),
//       );
//     }

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
//     30, // numCrew
//   );
// });

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

// export const mintAndPrepareCrew = async (
//     crewMerkleTree: PublicKey,
//     crewOwner: PublicKey,
//     umi: Umi,
//     knownLeaves: UmiPublicKey[] = [],
//     numCrew = 5,
//     startingLeafIndex?: number,
//     treeCreatorOrDelegate?: AsyncSigner,
//   ) => {
//     const umiCrewMerkleTree = fromWeb3JsPublicKey(crewMerkleTree);
//     const merkleTreeAccount = await fetchMerkleTree(umi, umiCrewMerkleTree);
//     const localKnownLeaves = [...knownLeaves];
//     const leafIndex = startingLeafIndex ?? localKnownLeaves.length;
//     let treeAdmin: UmiKeypair | undefined = undefined;
//     if (treeCreatorOrDelegate && treeCreatorOrDelegate.inner) {
//       treeAdmin = fromWeb3JsKeypair(treeCreatorOrDelegate.inner() as Keypair);
//     }
//     const items: (CrewTransferInput & { metadata: MetadataArgsArgs })[] = [];
//     for (let index = 0; index < numCrew; index++) {
//       const {
//         metadata,
//         leaf,
//         leafIndex: newLeafIndex,
//       } = await mintCNFT(umi, {
//         merkleTree: umiCrewMerkleTree,
//         leafOwner: fromWeb3JsPublicKey(crewOwner),
//         leafIndex: leafIndex + index,
//         treeCreatorOrDelegate:
//           treeAdmin && createSignerFromKeypair(umi, treeAdmin),
//       });
//       const newProof = getMerkleProof(
//         [...localKnownLeaves, leaf],
//         CREW_TREE_MAX_DEPTH,
//         leaf,
//         newLeafIndex,
//       );
//       localKnownLeaves.push(leaf);
//       items.push({
//         merkleTree: toWeb3JsPublicKey(umiCrewMerkleTree),
//         root: Array.from(getCurrentRoot(merkleTreeAccount.tree)),
//         dataHash: Array.from(hashMetadataData(metadata)),
//         leafIndex: newLeafIndex,
//         metadata,
//         creatorHash: new PublicKey(hashMetadataCreators(metadata.creators)),
//         proof: newProof.map((it) => toWeb3JsPublicKey(it)),
//       });
//     }

//     return {
//       items,
//       updatedLeaves: localKnownLeaves,
//     };
//   };
pub fn mock_crew_setup(_svm: &mut LiteSVM, _num_crew: usize) {
    // Create crew merkle tree account
    let crew_merkle_tree_kp = Keypair::new();
    let crew_merkle_tree_pk = crew_merkle_tree_kp.pubkey();

    // Create crew config account
    let crew_program_config_kp = Keypair::new();
    let crew_program_config_pk = crew_program_config_kp.pubkey();

    dbg!(&crew_merkle_tree_pk);
    dbg!(&crew_program_config_pk);
    todo!();
}

// // cargo test --test helpers -- --nocapture
// mod tests {
//     use super::*;

//     #[test]
//     fn test_setup_crew_config_instructions() {
//         todo!();
//     }

//         #[test]
//         fn test_mock_crew_setup() {
//             let mut svm = LiteSVM::default();

//             let num_crew = 15;
//             mock_crew_setup(&mut svm, num_crew);
//         }
// }
