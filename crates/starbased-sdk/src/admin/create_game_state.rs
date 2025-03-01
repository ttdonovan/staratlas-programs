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

use staratlas_sage::{
    instruction::InitGameState, typedefs::InitGameStateInput, ID as SAGE_PROGRAM_ID,
};

pub struct CreateGameState<'a> {
    authority_kp: &'a Keypair,
    profile_pk: &'a Pubkey,
    game_pk: &'a Pubkey,
    game_update_id: u64,
    key_index: u16,
    funder_kp: &'a Keypair,
}

impl<'a> CreateGameState<'a> {
    pub fn new(
        authority_kp: &'a Keypair,
        profile_pk: &'a Pubkey,
        game_pk: &'a Pubkey,
        funder_kp: &'a Keypair,
    ) -> Self {
        CreateGameState {
            authority_kp,
            profile_pk,
            game_pk,
            game_update_id: 0,
            key_index: 0,
            funder_kp,
        }
    }

    pub fn set_game_update_id(mut self, game_update_id: u64) -> Self {
        self.game_update_id = game_update_id;
        self
    }

    pub fn set_profile_key_index(mut self, key_index: u16) -> Self {
        self.key_index = key_index;
        self
    }

    pub fn send(self, svm: &mut LiteSVM) -> Result<Pubkey, FailedTransactionMetadata> {
        let authority_pk = self.authority_kp.pubkey();
        let funder_pk = self.funder_kp.pubkey();

        let (game_state_pda, _bump) = Pubkey::find_program_address(
            &[
                b"GameState",
                self.game_pk.as_ref(),
                &(self.game_update_id + 1).to_le_bytes(),
            ],
            &SAGE_PROGRAM_ID,
        );

        let ix = Instruction {
            program_id: SAGE_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new_readonly(authority_pk, true), // InitGameStateGameAndProfile<'info> pub key: Signer<'info>,
                AccountMeta::new_readonly(*self.profile_pk, false), // InitGameStateGameAndProfile<'info>pub profile: AccountInfo<'info>,
                AccountMeta::new_readonly(*self.game_pk, false), // InitGameStateGameAndProfile<'info> pub game_id: AccountInfo<'info>,
                AccountMeta::new(funder_pk, true),               // pub funder: Signer<'info>,
                AccountMeta::new(game_state_pda, false), // pub game_state: AccountInfo<'info>,
                AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
            ],
            data: InitGameState {
                _input: InitGameStateInput {
                    key_index: self.key_index,
                },
            }
            .data(),
        };

        let block_hash = svm.latest_blockhash();
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&funder_pk),
            &[self.authority_kp, self.funder_kp],
            block_hash,
        );
        svm.send_transaction(tx)?;

        Ok(game_state_pda)
    }
}
