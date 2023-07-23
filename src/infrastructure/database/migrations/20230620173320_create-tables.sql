BEGIN;

/* Create users table. Check `src/infrastructure/database/models/user.rs` */
CREATE TABLE users (
    id UUID NOT NULL,
    tg_id BIGINT NOT NULL,
    language_code VARCHAR,
    show_nsfw BOOLEAN DEFAULT false,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE (tg_id)
);

/* Create sources table. Check `src/infrastructure/database/models/source.rs` */
CREATE TABLE sources (
    id UUID NOT NULL,
    name VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE (name, url)
);

/* Create media table. Check `src/infrastructure/database/models/media.rs` */
CREATE TABLE media (
    id UUID NOT NULL,
    url VARCHAR NOT NULL,
    genre VARCHAR,
    media_type VARCHAR NOT NULL,
    is_sfw BOOLEAN,
    source_id UUID NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (source_id) REFERENCES sources (id) ON DELETE SET NULL ON UPDATE CASCADE,
    UNIQUE (url, genre)
);

/* Create user_media_views table. Check `src/infrastructure/database/models/user_media_view.rs` */
CREATE TABLE user_media_views (
    id UUID NOT NULL,
    user_id UUID NOT NULL,
    media_id UUID NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (media_id) REFERENCES media (id) ON DELETE SET NULL ON UPDATE CASCADE,
    UNIQUE (user_id, media_id)
);

COMMIT;