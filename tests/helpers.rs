use litesvm::LiteSVM;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

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
    fn test_mock_crew_setup() {
        let mut svm = LiteSVM::default();

        let num_crew = 15;
        mock_crew_setup(&mut svm, num_crew);
    }
}