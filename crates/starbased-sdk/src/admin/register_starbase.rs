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
    instruction::RegisterStarbase as ixRegisterStarbase, typedefs::RegisterStarbaseInputUnpacked,
    ID as SAGE_PROGRAM_ID,
};

pub struct RegisterStarbase<'a> {
    authority_kp: &'a Keypair,
    profile_pk: &'a Pubkey,
    game_pk: &'a Pubkey,
    game_state_pk: &'a Pubkey,
    sector_pk: &'a Pubkey,
    coordinates: [i64; 2],
    name: [u8; 64],
    sub_coordinates: [i64; 2],
    starbase_level_index: u8,
    faction: u8,
    key_index: u16,
    funder_kp: &'a Keypair,
}

impl<'a> RegisterStarbase<'a> {
    pub fn new(
        authority_kp: &'a Keypair,
        profile_pk: &'a Pubkey,
        game_pk: &'a Pubkey,
        game_state_pk: &'a Pubkey,
        sector_pk: &'a Pubkey,
        funder_kp: &'a Keypair,
    ) -> Self {
        RegisterStarbase {
            authority_kp,
            profile_pk,
            game_pk,
            game_state_pk,
            sector_pk,
            coordinates: [0, 0],
            name: [0u8; 64],
            sub_coordinates: [0, 0],
            faction: 0,
            starbase_level_index: 0,
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

    pub fn set_sub_coordinates(mut self, sub_coordinates: [i64; 2]) -> Self {
        self.sub_coordinates = sub_coordinates;
        self
    }

    pub fn set_faction(mut self, faction: u8) -> Self {
        self.faction = faction;
        self
    }

    pub fn set_starbase_level_index(mut self, starbase_level_index: u8) -> Self {
        self.starbase_level_index = starbase_level_index;
        self
    }

    pub fn set_profile_key_index(mut self, key_index: u16) -> Self {
        self.key_index = key_index;
        self
    }

    pub fn send(self, svm: &mut LiteSVM) -> Result<Pubkey, FailedTransactionMetadata> {
        let authority_pk = self.authority_kp.pubkey();
        let funder_pk = self.funder_kp.pubkey();

        let (starbase_pda, _bump) = Pubkey::find_program_address(
            &[
                b"Starbase",
                self.game_pk.as_ref(),
                &self.coordinates[0].to_le_bytes(),
                &self.coordinates[1].to_le_bytes(),
            ],
            &SAGE_PROGRAM_ID,
        );

        let ix = Instruction {
            program_id: SAGE_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(funder_pk, true),     // pub funder: Signer<'info>,
                AccountMeta::new(starbase_pda, false), // pub starbase: AccountInfo<'info>,
                AccountMeta::new_readonly(*self.sector_pk, false), // pub sector: AccountInfo<'info>
                AccountMeta::new_readonly(authority_pk, true), // RegisterStarbaseGameStateAndProfileGameAndProfile<'info> pub key: Signer<'info>,
                AccountMeta::new_readonly(*self.profile_pk, false), // RegisterStarbaseGameStateAndProfileGameAndProfile<'info> pub profile: AccountInfo<'info>,
                AccountMeta::new_readonly(*self.game_pk, false), // RegisterStarbaseGameStateAndProfileGameAndProfile<'info> pub game_id: AccountInfo<'info>,
                AccountMeta::new_readonly(*self.game_state_pk, false), // pub game_state: AccountInfo<'info>,
                AccountMeta::new_readonly(system_program::ID, false), // pub system_program: AccountInfo<'info>,
            ],
            data: ixRegisterStarbase {
                _input: RegisterStarbaseInputUnpacked {
                    name: self.name,
                    sub_coordinates: self.sub_coordinates,
                    starbase_level_index: self.starbase_level_index,
                    faction: self.faction as u8,
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

        Ok(starbase_pda)
    }
}
