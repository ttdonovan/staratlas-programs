use litesvm::LiteSVM;
use solana_sdk::{
    feature_set::FeatureSet,
    native_token::LAMPORTS_PER_SOL,
    program_pack::Pack,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account_client::{
    address::get_associated_token_address_with_program_id,
    instruction::create_associated_token_account,
};
use spl_token::{instruction::mint_to, state::Account as TokenAccount};

mod shared;
use shared::{
    constants::{
        CREW_PROGRAM_BYTES, CREW_PROGRAM_ID, PLAYER_PROFILE_PROGRAM_BYTES,
        PLAYER_PROFILE_PROGRAM_ID,
    },
    helpers,
};

// cargo test --test crew_test -- --nocapture
#[test]
fn crew_test() {
    let feature_set = FeatureSet::all_enabled();
    let mut svm = LiteSVM::default()
        .with_feature_set(feature_set)
        .with_builtins()
        .with_lamports(LAMPORTS_PER_SOL * 1000)
        .with_sysvars()
        .with_spl_programs();

    svm.add_program(CREW_PROGRAM_ID, CREW_PROGRAM_BYTES);
    svm.add_program(PLAYER_PROFILE_PROGRAM_ID, PLAYER_PROFILE_PROGRAM_BYTES);

    let funder_kp = Keypair::new();
    let funder_pk = funder_kp.pubkey();
    svm.airdrop(&funder_pk, LAMPORTS_PER_SOL * 10).unwrap();
    dbg!(&funder_pk);

    let admin_kp = Keypair::new();
    let admin_pk = admin_kp.pubkey();
    dbg!(&admin_pk);

    let crew_pack_mint_kp = Keypair::new();
    let crew_pack_mint_pk =
        helpers::create_mint(&mut svm, &funder_kp, &crew_pack_mint_kp, 0, &admin_pk).unwrap();
    dbg!(&crew_pack_mint_pk);
    assert_eq!(crew_pack_mint_pk, crew_pack_mint_kp.pubkey());

    let ata_ix = create_associated_token_account(
        &funder_pk,
        &admin_pk,
        &crew_pack_mint_pk,
        &spl_token::id(),
    );

    let tx = Transaction::new_signed_with_payer(
        &[ata_ix],
        Some(&funder_pk),
        &[&funder_kp],
        svm.latest_blockhash(),
    );
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    let ata = get_associated_token_address_with_program_id(
        &admin_pk,
        &crew_pack_mint_pk,
        &spl_token::id(),
    );
    dbg!(&ata);

    let mint_ix = mint_to(
        &spl_token::id(),
        &crew_pack_mint_pk,
        &ata,
        &admin_pk,
        &[&admin_pk],
        10,
    )
    .unwrap();

    let block_hash = svm.latest_blockhash();
    let mut tx = Transaction::new_with_payer(&[mint_ix], Some(&funder_pk));
    tx.partial_sign(&[&funder_kp], block_hash);
    tx.partial_sign(&[&admin_kp], block_hash);

    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_ok());

    let token_acc = svm.get_account(&ata).unwrap();
    let token_data = TokenAccount::unpack(&token_acc.data).unwrap();
    assert_eq!(token_data.amount, 10);

    dbg!(&token_acc);
    dbg!(&token_data);

    // https://docs.rs/spl-account-compression/latest/spl_account_compression/
    // https://docs.rs/solana-merkle-tree/2.2.0/solana_merkle_tree/index.html

    assert!(true);
}
