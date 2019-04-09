CREATE TABLE Events
(
    seq_num bigserial NOT NULL,
    aggregate_id bigint NOT NULL,
    data jsonb NOT NULL,
    type varchar(255) NOT NULL,
    meta jsonb NOT NULL,
    log_date timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY (seq_num)
);

CREATE INDEX idx_events_aggregate_id ON Events (aggregate_id);