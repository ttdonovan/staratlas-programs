anchor_gen::generate_cpi_crate!("sage.json");
anchor_lang::declare_id!("SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE");

pub mod state_with_data {
    use crate::{state, typedefs};
    use anchor_lang::prelude::borsh;

    #[derive(Debug)]
    pub enum FleetState {
        StarbaseLoadingBay(typedefs::StarbaseLoadingBay),
        Idle(typedefs::Idle),
        MineAsteroid(typedefs::MineAsteroid),
        MoveWarp(typedefs::MoveWarp),
        MoveSubwarp(typedefs::MoveSubwarp),
        Respawn(typedefs::Respawn),
    }

    impl borsh::BorshDeserialize for FleetState {
        fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
            let discriminator = u8::deserialize_reader(reader)?;
            match discriminator {
                0 => Ok(FleetState::StarbaseLoadingBay(
                    typedefs::StarbaseLoadingBay::deserialize_reader(reader)?,
                )),
                1 => Ok(FleetState::Idle(typedefs::Idle::deserialize_reader(
                    reader,
                )?)),
                2 => Ok(FleetState::MineAsteroid(
                    typedefs::MineAsteroid::deserialize_reader(reader)?,
                )),
                3 => Ok(FleetState::MoveWarp(
                    typedefs::MoveWarp::deserialize_reader(reader)?,
                )),
                4 => Ok(FleetState::MoveSubwarp(
                    typedefs::MoveSubwarp::deserialize_reader(reader)?,
                )),
                5 => Ok(FleetState::Respawn(typedefs::Respawn::deserialize_reader(
                    reader,
                )?)),
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid FleetState discriminator: {}", discriminator),
                )),
            }
        }
    }

    pub struct FleetWithState(pub state::Fleet, pub FleetState);

    impl borsh::BorshDeserialize for FleetWithState {
        fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
            // Skip the 8-byte discriminator
            let mut discriminator = [0u8; 8];
            reader.read_exact(&mut discriminator)?;

            // Deserialize the Fleet state
            let fleet = state::Fleet::deserialize_reader(reader)?;

            // Deserialize the FleetState
            let fleet_state = FleetState::deserialize_reader(reader)?;

            Ok(FleetWithState(fleet, fleet_state))
        }
    }
}
