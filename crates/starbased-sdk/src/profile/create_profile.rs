use anchor_lang::{prelude::Pubkey as AnchorPubkey, InstructionData};
use litesvm::{types::FailedTransactionMetadata, LiteSVM};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};

// use staratlas_sage::{instruction::InitGame, ID as SAGE_PROGRAM_ID};
use staratlas_player_profile::{
    instruction::CreateProfile as ixCreateProfile, typedefs::AddKeyInput,
    ID as PLAYER_PROFILE_PROGRAM_ID,
};

pub struct CreateProfile<'a> {
    profile_kp: &'a Keypair,
    funder_kp: &'a Keypair,
}

impl<'a> CreateProfile<'a> {
    pub fn new(profile_kp: &'a Keypair, funder_kp: &'a Keypair) -> Self {
        CreateProfile {
            profile_kp,
            funder_kp,
        }
    }

    pub fn send(self, svm: &mut LiteSVM) -> Result<Pubkey, FailedTransactionMetadata> {
        let funder_pk = self.funder_kp.pubkey();
        let profile_pk = self.profile_kp.pubkey();

        let ix = Instruction {
            program_id: PLAYER_PROFILE_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(funder_pk, true),
                AccountMeta::new(profile_pk, true),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new(profile_pk, true), // auth authority (writable, signer)
            ],
            data: ixCreateProfile {
                _key_permissions: vec![
                    // Auth Authority - Full permissions
                    AddKeyInput {
                        scope: AnchorPubkey::new_from_array(PLAYER_PROFILE_PROGRAM_ID.to_bytes()),
                        expire_time: -1,
                        permissions: [0xFF, 0xFF, 0, 0, 0, 0, 0, 0], // All permissions enabled
                    },
                ],
                _key_threshold: 1,
            }
            .data(),
        };

        let block_hash = svm.latest_blockhash();
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&funder_pk),
            &[self.profile_kp, self.funder_kp],
            block_hash,
        );
        svm.send_transaction(tx)?;

        Ok(profile_pk)
    }
}
