use anchor_lang::InstructionData;
use litesvm::{types::FailedTransactionMetadata, LiteSVM};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};

use staratlas_sage::{instruction::InitGame, ID as SAGE_PROGRAM_ID};

pub struct CreateGame<'a> {
    authority_kp: &'a Keypair,
    profile_pk: &'a Pubkey,
    funder_kp: &'a Keypair,
    game_kp: Option<Keypair>,
}

impl<'a> CreateGame<'a> {
    pub fn new(authority_kp: &'a Keypair, profile_pk: &'a Pubkey, funder_kp: &'a Keypair) -> Self {
        CreateGame {
            authority_kp,
            profile_pk,
            funder_kp,
            game_kp: None,
        }
    }

    pub fn set_game_kp(mut self, game_kp: Keypair) -> Self {
        self.game_kp = Some(game_kp);
        self
    }

    pub fn send(self, svm: &mut LiteSVM) -> Result<Pubkey, FailedTransactionMetadata> {
        let authority_pk = self.authority_kp.pubkey();
        let funder_pk = self.funder_kp.pubkey();

        let game_kp = self.game_kp.unwrap_or(Keypair::new());
        let game_pk = game_kp.pubkey();

        let ix = Instruction {
            program_id: SAGE_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(authority_pk, true),
                AccountMeta::new_readonly(*self.profile_pk, false),
                AccountMeta::new(funder_pk, true),
                AccountMeta::new(game_pk, true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: InitGame {}.data(),
        };

        let block_hash = svm.latest_blockhash();
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&funder_pk),
            &[self.authority_kp, self.funder_kp, &game_kp],
            block_hash,
        );
        svm.send_transaction(tx)?;

        Ok(game_pk)
    }
}
