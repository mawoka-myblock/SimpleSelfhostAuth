-- Your SQL goes here

CREATE TABLE users
(
    id          uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    username    text UNIQUE      NOT NULL,
    password    text             NOT NULL,
    profile_pic text             NULL            DEFAULT '',
    email       text UNIQUE      NOT NULL,
    verified    boolean                          DEFAULT false,
    created_at  timestamp        NOT NULL        DEFAULT now(),
    admin       boolean          NOT NULL        DEFAULT false

);
INSERT INTO users(username, password, email, admin)
VALUES ('admin', '$argon2i$v=19$m=16,t=2,p=1$OXc1dlR0VEtNcTRjRTNrSw$mEGzJZ3o6bo0N1dPaGR2kw', 'admin@admin.com', true)
ON CONFLICT DO NOTHING;

CREATE TABLE apps
(
    id             uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    name           text             NOT NULL,
    description    text             NULL,
    owner          uuid             NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at     timestamp        NOT NULL        DEFAULT NOW(),
    updated_at     timestamp        NOT NULL        DEFAULT NOW(),
    token_lifetime integer          NOT NULL        DEFAULT 3600,
    domains        text[]           NOT NULL
)