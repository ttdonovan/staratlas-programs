anchor_gen::generate_cpi_interface!(idl_path = "sage.json");

impl std::fmt::Debug for state::Fleet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Fleet {{ owner_profile: {}, fleet_ships: {}}}",
            self.owner_profile.to_string(),
            self.fleet_ships.to_string()
        )
    }
}

pub mod ui;

declare_id!("SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE");
