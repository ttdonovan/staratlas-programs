use anchor_lang::InstructionData;
use litesvm::{types::FailedTransactionMetadata, LiteSVM};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};

use staratlas_sage::{
    instruction::ActivateGameState as ixActivateGameState, typedefs::ManageGameInput,
    ID as SAGE_PROGRAM_ID,
};

pub struct ActivateGameState<'a> {
    authority_kp: &'a Keypair,
    profile_pk: &'a Pubkey,
    game_pk: &'a Pubkey,
    game_state_pk: &'a Pubkey,
    key_index: u16,
    payer_kp: &'a Keypair,
}

impl<'a> ActivateGameState<'a> {
    pub fn new(
        authority_kp: &'a Keypair,
        profile_pk: &'a Pubkey,
        game_pk: &'a Pubkey,
        game_state_pk: &'a Pubkey,
        payer_kp: &'a Keypair,
    ) -> Self {
        ActivateGameState {
            authority_kp,
            profile_pk,
            game_pk,
            game_state_pk,
            key_index: 0,
            payer_kp,
        }
    }

    pub fn set_profile_key_index(mut self, key_index: u16) -> Self {
        self.key_index = key_index;
        self
    }

    pub fn send(self, svm: &mut LiteSVM) -> Result<Pubkey, FailedTransactionMetadata> {
        let authority_pk = self.authority_kp.pubkey();
        let payer_pk = self.payer_kp.pubkey();

        let ix = Instruction {
            program_id: SAGE_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new_readonly(authority_pk, true), // ActivateGameStateGameAndProfile<'info> pub key: Signer<'info>,
                AccountMeta::new_readonly(*self.profile_pk, false), // ActivateGameStateGameAndProfile<'info>pub profile: AccountInfo<'info>,
                AccountMeta::new(*self.game_pk, false), // ActivateGameStateGameAndProfile<'info> pub game_id: AccountInfo<'info>,
                AccountMeta::new(*self.game_state_pk, false), // pub game_state: AccountInfo<'info>,
            ],
            data: ixActivateGameState {
                _input: ManageGameInput {
                    key_index: self.key_index,
                },
            }
            .data(),
        };

        let block_hash = svm.latest_blockhash();
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer_pk),
            &[self.authority_kp, self.payer_kp],
            block_hash,
        );
        svm.send_transaction(tx)?;

        Ok(*self.game_state_pk)
    }
}
