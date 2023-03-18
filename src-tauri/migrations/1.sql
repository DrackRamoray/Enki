CREATE TABLE IF NOT EXISTS kv
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name       TEXT                              NOT NULL,
    value      TEXT                              NOT NULL,
    created_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS topic
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title      TEXT                              NOT NULL,
    category   TEXT                              NOT NULL,
    status     INTEGER                           NOT NULL DEFAULT 1,
    created_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    role       TEXT                              NOT NULL DEFAULT 'user',
    message    TEXT                              NOT NULL,
    topic_id   INTEGER                           NOT NULL,
    created_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (topic_id) REFERENCES topic (id)
);

CREATE TABLE IF NOT EXISTS chat_response
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    role       TEXT                              NOT NULL,
    raw        TEXT                              NOT NULL,
    markdown   TEXT                              NOT NULL,
    chat_id    INTEGER                           NOT NULL,
    topic_id   INTEGER                           NOT NULL,
    created_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (chat_id) REFERENCES chat (id),
    FOREIGN KEY (topic_id) REFERENCES topic (id)
);

CREATE TABLE IF NOT EXISTS image
(
    id             INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    message        TEXT,
    original_image TEXT,
    mask           TEXT,
    category       TEXT                              NOT NULL,
    topic_id       INTEGER                           NOT NULL,
    created_at     TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (topic_id) REFERENCES topic (id)
);

CREATE TABLE IF NOT EXISTS image_response
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    uri        TEXT                              NOT NULL,
    size       TEXT                              NOT NULL,
    image_id   INTEGER                           NOT NULL,
    topic_id   INTEGER                           NOT NULL,
    created_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (image_id) REFERENCES image (id),
    FOREIGN KEY (topic_id) REFERENCES topic (id)
);

CREATE TABLE IF NOT EXISTS audio
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    file       TEXT                              NOT NULL,
    message    TEXT                              NOT NULL,
    response   TEXT                              NOT NULL DEFAULT '',
    topic_id   INTEGER                           NOT NULL,
    created_at TIMESTAMP                         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (topic_id) REFERENCES topic (id)
);
