CREATE TABLE IF NOT EXISTS staratlas_sage_accounts (
    pubkey VARCHAR(32) PRIMARY KEY,
    owner VARCHAR(32) NOT NULL,
    data BLOB NOT NULL,
    space INTEGER NOT NULL,
    slot INTEGER NOT NULL
);
