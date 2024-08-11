-- Your SQL goes here
CREATE TABLE journal (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    "transaction" INTEGER NOT NULL,
    account INTEGER NOT NULL,
    credit_amount FLOAT NOT NULL,
    debit_amount FLOAT NOT NULL
);