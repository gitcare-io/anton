table! {
    dmrprojections (id) {
        id -> Varchar,
        repo_id -> Int8,
        from -> Timestamp,
        to -> Timestamp,
        data -> Jsonb,
        projected_at -> Timestamp,
    }
}
