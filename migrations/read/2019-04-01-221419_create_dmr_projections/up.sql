CREATE TABLE DMRProjections
(
    id varchar(255) NOT NULL,
    repo_id bigint NOT NULL,
    "from" timestamp NOT NULL,
    "to" timestamp NOT NULL,
    data jsonb NOT NULL,
    projected_at timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY (id)
);