CREATE TABLE IF NOT EXISTS staratlas_fleet_rentals_accounts (
    id INTEGER PRIMARY KEY,
    pubkey VARCHAR(32) UNIQUE NOT NULL,
    lamports BIGINT NOT NULL,
    data BLOB NOT NULL,
    owner VARCHAR(32) NOT NULL,
    executable BOOL NOT NULL,
    rent_epoch BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS staratlas_sage_accounts (
    id INTEGER PRIMARY KEY,
    pubkey VARCHAR(32) UNIQUE NOT NULL,
    lamports BIGINT NOT NULL,
    data BLOB NOT NULL,
    owner VARCHAR(32) NOT NULL,
    executable BOOL NOT NULL,
    rent_epoch BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS rental_contract_states (
    id INTEGER PRIMARY KEY,
    pubkey VARCHAR(32) UNIQUE NOT NULL,
    fleet VARCHAR(32),
    rate BIGINT NOT NULL,
    current_rental_state VARCHAR(32) NOT NULL,
    owner_profile VARCHAR(32) NOT NULL
);

CREATE TABLE IF NOT EXISTS sage_fleets (
    id INTEGER PRIMARY KEY,
    pubkey VARCHAR(32) UNIQUE NOT NULL,
    game_id VARCHAR(32) NOT NULL,
    owner_profile VARCHAR(32) NOT NULL,
    fleet_ships VARCHAR(32) NOT NULL,
    faction INTEGER NOT NULL,
    fleet_label VARCHAR(32) NOT NULL,
    ship_counts TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sage_fleet_ships (
    id INTEGER PRIMARY KEY,
    pubkey VARCHAR(32) NOT NULL,
    fleet VARCHAR(32) NOT NULL,
    idx INTEGER NOT NULL,
    ship VARCHAR(32) NOT NULL,
    amount BIGINT NOT NULL,
    fleet_ships_info_count INTEGER NOT NULL,
    UNIQUE (pubkey, fleet, idx)
);

CREATE TABLE IF NOT EXISTS sage_ships (
    id INTEGER PRIMARY KEY,
    pubkey VARCHAR(32) UNIQUE NOT NULL,
    game_id VARCHAR(32) NOT NULL,
    mint VARCHAR(32) NOT NULL,
    name VARCHAR(64) NOT NULL,
    size_class INTEGER NOT NULL
);
