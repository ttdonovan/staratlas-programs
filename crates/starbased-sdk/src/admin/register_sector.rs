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

use staratlas_sage::{instruction::RegisterSector as ixRegisterSector, ID as SAGE_PROGRAM_ID};

pub struct RegisterSector<'a> {
    authority_kp: &'a Keypair,
    profile_pk: &'a Pubkey,
    discoverer_pk: &'a Pubkey,
    game_pk: &'a Pubkey,
    coordinates: [i64; 2],
    name: [u8; 64],
    key_index: u16,
    funder_kp: &'a Keypair,
}

impl<'a> RegisterSector<'a> {
    pub fn new(
        authority_kp: &'a Keypair,
        profile_pk: &'a Pubkey,
        discoverer_pk: &'a Pubkey,
        game_pk: &'a Pubkey,
        funder_kp: &'a Keypair,
    ) -> Self {
        RegisterSector {
            authority_kp,
            profile_pk,
            discoverer_pk,
            game_pk,
            coordinates: [0, 0],
            name: [0u8; 64],
            key_index: 0,
            funder_kp,
        }
    }

    pub fn set_coordinates(mut self, coordinates: [i64; 2]) -> Self {
        self.coordinates = coordinates;
        self
    }

    pub fn set_name(mut self, name: String) -> Self {
        let name_bytes = name.as_bytes();
        let mut name = [0u8; 64];
        name[..name_bytes.len()].copy_from_slice(name_bytes);

        self.name = name;
        self
    }

    pub fn set_profile_key_index(mut self, key_index: u16) -> Self {
        self.key_index = key_index;
        self
    }

    pub fn send(self, svm: &mut LiteSVM) -> Result<Pubkey, FailedTransactionMetadata> {
        let authority_pk = self.authority_kp.pubkey();
        let funder_pk = self.funder_kp.pubkey();

        let (sector_pda, _bump) = Pubkey::find_program_address(
            &[
                b"Sector",
                self.game_pk.as_ref(),
                &(self.coordinates[0]).to_le_bytes(),
                &(self.coordinates[1]).to_le_bytes(),
            ],
            &SAGE_PROGRAM_ID,
        );

        let ix = Instruction {
            program_id: SAGE_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new_readonly(authority_pk, true), // RegisterSectorGameAndProfile<'info> pub key: Signer<'info>,
                AccountMeta::new_readonly(*self.profile_pk, false), // RegisterSectorGameAndProfile<'info>pub profile: AccountInfo<'info>,
                AccountMeta::new(*self.game_pk, false), // RegisterSectorGameAndProfile<'info> pub game_id: AccountInfo<'info>,
                AccountMeta::new(funder_pk, true),      // pub funder: Signer<'info>,
                AccountMeta::new_readonly(*self.discoverer_pk, false), // pub discoverer: AccountInfo<'info>,
                AccountMeta::new(sector_pda, false), // pub sector: AccountInfo<'info>
                AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
            ],
            data: ixRegisterSector {
                _coordinates: self.coordinates,
                _name: self.name,
                _key_index: self.key_index,
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

        Ok(sector_pda)
    }
}
