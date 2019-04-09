table! {
    dmrprojections (id) {
        id -> Varchar,
        aggregate_id -> Int8,
        aggregate_type -> Varchar,
        from -> Nullable<Timestamp>,
        to -> Nullable<Timestamp>,
        data -> Jsonb,
        projected_at -> Timestamp,
    }
}
