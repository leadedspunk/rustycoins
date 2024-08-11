-- Your SQL goes here
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    "date" DATE NOT NULL,
    "description" VARCHAR,
    amount FLOAT NOT NULL,
    credit_account INTEGER NOT NULL,
    debit_account INTEGER NOT NULL
);