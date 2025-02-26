use litesvm::LiteSVM;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

pub fn setup_crew_config_instructions(
    authority: &'Signer Keypair,
    profile: &Pubkey,
    key_index: u8,
    game_id: &Pubkey,
) {
    let crew_collection_kp = Keypair::generate();
    let crew_creator_kp = Keypair::generate();

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

    todo!();
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
pub fn mock_crew_setup(svm: &mut LiteSVM, num_crew: usize) {
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

// cargo test --test helpers -- --nocapture
mod tests {
    use super::*;

    #[test]
    fn test_setup_crew_config_instructions() {
        setup_crew_config_instructions();
    }

    //     #[test]
    //     fn test_mock_crew_setup() {
    //         let mut svm = LiteSVM::default();

    //         let num_crew = 15;
    //         mock_crew_setup(&mut svm, num_crew);
    //     }
}
