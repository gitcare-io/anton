CREATE TABLE DMRProjections
(
    id varchar(255) NOT NULL,
    aggregate_id bigint NOT NULL,
    aggregate_type varchar(255) NOT NULL,
    "from" timestamp,
    "to" timestamp,
    data jsonb NOT NULL,
    projected_at timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY (id)
);

CREATE INDEX idx_dmrprojections_aggregate_id ON DMRProjections (aggregate_id);
CREATE INDEX idx_dmrprojections_from ON DMRProjections ("from");
CREATE INDEX idx_dmrprojections_to ON DMRProjections ("to");