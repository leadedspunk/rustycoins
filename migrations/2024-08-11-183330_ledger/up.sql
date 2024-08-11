-- Your SQL goes here
CREATE TABLE ledger (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account INTEGER NOT NULL,
    "transaction" INTEGER NOT NULL,
    "date" DATE NOT NULL,
    "description" VARCHAR,
    credit_amount FLOAT NOT NULL,
    debit_amount FLOAT NOT NULL,
    balance FLOAT NOT NULL
);