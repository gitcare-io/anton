table! {
    events (seq_num) {
        seq_num -> Int8,
        aggregate_id -> Int8,
        data -> Jsonb,
        #[sql_name = "type"]
        type_ -> Varchar,
        meta -> Jsonb,
        log_date -> Timestamp,
    }
}
