CREATE TABLE IF NOT EXISTS staratlas_sage_accounts (
    pubkey TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    data BLOB NOT NULL,
    space INTEGER NOT NULL,
    slot INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS sage_ui_fleets (
    pubkey TEXT NOT NULL PRIMARY KEY,
    owner TEXT NOT NULL,
    data TEXT NOT NULL
);
