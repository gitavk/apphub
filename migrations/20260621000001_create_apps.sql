CREATE TABLE apps (
    id          UUID        PRIMARY KEY DEFAULT uuidv7(),
    bundle_id   TEXT        NOT NULL UNIQUE,
    name        TEXT        NOT NULL,
    developer   TEXT        NOT NULL,
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX apps_created_at_idx ON apps (created_at DESC);
