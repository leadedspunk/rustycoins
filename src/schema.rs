// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Nullable<Integer>,
        name -> Text,
        parent -> Integer,
    }
}

diesel::table! {
    journal (id) {
        id -> Nullable<Integer>,
        transaction -> Integer,
        account -> Integer,
        credit_amount -> Float,
        debit_amount -> Float,
    }
}

diesel::table! {
    ledger (id) {
        id -> Nullable<Integer>,
        account -> Integer,
        transaction -> Integer,
        date -> Date,
        description -> Nullable<Text>,
        credit_amount -> Float,
        debit_amount -> Float,
        balance -> Float,
    }
}

diesel::table! {
    transactions (id) {
        id -> Nullable<Integer>,
        date -> Date,
        description -> Nullable<Text>,
        amount -> Float,
        credit_account -> Integer,
        debit_account -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    journal,
    ledger,
    transactions,
);
