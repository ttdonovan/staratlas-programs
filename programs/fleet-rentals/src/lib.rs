// anchor_gen::generate_cpi_crate!("fleet_rentals.json");
anchor_lang::declare_id!("SRSLY1fq9TJqCk1gNSE7VZL2bztvTn9wm4VR8u8jMKT");

pub mod state {
    use anchor_lang::prelude::{AnchorDeserialize, Pubkey, borsh};

    #[derive(Debug, AnchorDeserialize)]
    pub struct ContractState {
        /// The account version (typically set to 1).
        pub version: u8,
        /// Flag indicating if the contract is scheduled for closure.
        pub to_close: bool,
        /// The rental price per period (converted from Stardust to ATLAS).
        pub rate: u64,
        /// The minimum duration allowed for the rental session.
        pub duration_min: u64,
        /// The maximum allowable rental duration.
        pub duration_max: u64,
        /// Enum representing the payment frequency (Daily)
        pub payment_freq: u8, // PaymentFrequency
        /// Public key representing the fleet asset being rented.
        pub fleet: Pubkey,
        /// Identifier for the associated game.
        pub game_id: Pubkey,
        /// Tracks the PDA of the active rental state. Defaults to system program’s ID when free.
        pub current_rental_state: Pubkey,
        /// The fleet owner’s public key.
        pub owner: Pubkey,
        /// Owner’s token account used for receiving rental payments.
        pub owner_token_account: Pubkey,
        /// The owner’s profile account from the Sage program.
        pub owner_profile: Pubkey,
        /// The bump seed value from the PDA derivation.
        pub bump: u8,
    }
}

pub mod seeds {
    pub const CONTRACT_STATE_SEED: &[u8] = b"rental_contract"; // ["rental_contract", fleet.publicKey]
    pub const CONTRACT_STATE_DISCRIMINATOR: [u8; 8] = [190, 138, 10, 223, 189, 116, 222, 115];

    pub const FLEET_DISCRIMINATOR: [u8; 8] = [109, 207, 251, 48, 106, 2, 136, 163];

    pub const RENTAL_STATE_SEED: &[u8] = b"rental_state"; // ["rental_state", contractPDA, borrower.publicKey]
    pub const RENTAL_STATE_DISCRIMINATOR: [u8; 8] = [97, 162, 29, 222, 251, 251, 180, 244];

    pub const RENTAL_DISCRIMINATOR: [u8; 8] = [186, 27, 154, 111, 51, 36, 159, 90];
}
