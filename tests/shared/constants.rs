#![allow(dead_code)]
use solana_sdk::{pubkey, pubkey::Pubkey};

pub const CARGO_PROGRAM_ID: Pubkey = pubkey!("Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk");
pub const CARGO_PROGRAM_BYTES: &'static [u8; 608048] =
    include_bytes!("../../programs/cargo/Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk.so");

pub const CREW_PROGRAM_ID: Pubkey = pubkey!("CREWiq8qbxvo4SKkAFpVnc6t7CRQC4tAAscsNAENXgrJ");
pub const CREW_PROGRAM_BYTES: &'static [u8; 647904] =
    include_bytes!("../../programs/crew/CREWiq8qbxvo4SKkAFpVnc6t7CRQC4tAAscsNAENXgrJ.so");

pub const PLAYER_PROFILE_PROGRAM_ID: Pubkey =
    pubkey!("pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9");
pub const PLAYER_PROFILE_PROGRAM_BYTES: &'static [u8; 1174816] =
    include_bytes!("../../programs/player-profile/pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9.so");

pub const PROFILE_FACTION_PROGRAM_ID: Pubkey =
    pubkey!("pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq");
pub const PROFILE_FACTION_PROGRAM_BYTES: &'static [u8; 535312] =
    include_bytes!("../../programs/profile-faction/pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq.so");

pub const SAGE_PROGRAM_ID: Pubkey = pubkey!("SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE");
pub const SAGE_PROGRAM_BYTES: &'static [u8; 3232680] =
    include_bytes!("../../programs/sage/SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE.so");
