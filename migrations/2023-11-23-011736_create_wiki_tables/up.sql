-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE words
(
    id               SERIAL PRIMARY KEY,
    pos              TEXT,
    word             TEXT,
    lang_code        TEXT,
    lang             TEXT,
    etymology_number INT NULL,
    etymology_text   TEXT
);

CREATE TABLE forms
(
    id      SERIAL PRIMARY KEY,
    word_id INT REFERENCES words (id),
    form    TEXT
);

CREATE TABLE tags
(
    id      SERIAL PRIMARY KEY,
    form_id INT REFERENCES forms (id),
    tag     TEXT
);

CREATE TABLE etymology_templates
(
    id        SERIAL PRIMARY KEY,
    word_id   INT REFERENCES words (id),
    name      TEXT,
    expansion TEXT
);

CREATE TABLE args
(
    id                    SERIAL PRIMARY KEY,
    etymology_template_id INT REFERENCES etymology_templates (id),
    arg_key               TEXT,
    arg_val               TEXT
);

CREATE TABLE sounds
(
    id        SERIAL PRIMARY KEY,
    word_id   INT REFERENCES words (id),
    ipa       TEXT,
    enpr      TEXT,
    audio     TEXT,
    text      TEXT,
    ogg_url   TEXT,
    mp3_url   TEXT,
    homophone TEXT,
    rhymes    TEXT
);

CREATE TABLE sound_tags
(
    id       SERIAL PRIMARY KEY,
    sound_id INT REFERENCES sounds (id),
    tag      TEXT
);

CREATE TABLE hyphenations
(
    id      SERIAL PRIMARY KEY,
    word_id INT REFERENCES words (id),
    hyphen  TEXT
);

CREATE TABLE derived_words
(
    id      SERIAL PRIMARY KEY,
    word_id INT REFERENCES words (id),
    word    TEXT,
    _dis1   TEXT
);

CREATE TABLE senses
(
    id      SERIAL PRIMARY KEY,
    word_id INT REFERENCES words (id),
    id_1    TEXT
);

CREATE TABLE links
(
    id       SERIAL PRIMARY KEY,
    sense_id INT REFERENCES senses (id),
    link_1   TEXT,
    link_2   TEXT
);

CREATE TABLE raw_glosss
(
    id        SERIAL PRIMARY KEY,
    sense_id  INT REFERENCES senses (id),
    raw_gloss TEXT
);

CREATE TABLE glosss
(
    id       SERIAL PRIMARY KEY,
    sense_id INT REFERENCES senses (id),
    gloss    TEXT
);

CREATE TABLE sense_tags
(
    id       SERIAL PRIMARY KEY,
    sense_id INT REFERENCES senses (id),
    tag      TEXT
);

CREATE TABLE categorys
(
    id       SERIAL PRIMARY KEY,
    sense_id INT REFERENCES senses (id),
    name     TEXT,
    kind     TEXT,
    source   TEXT,
    orig     TEXT,
    langcode TEXT,
    _dis     TEXT
);

CREATE TABLE category_parents
(
    id          SERIAL PRIMARY KEY,
    category_id INT REFERENCES categorys (id),
    parent      TEXT
);

CREATE INDEX idx_words_word
    ON words (word);

CREATE INDEX idx_forms_form
    ON forms (form);

CREATE INDEX idx_etymology_templates_name
    ON etymology_templates (name);

CREATE INDEX idx_sounds_ipa
    ON sounds (ipa);

CREATE INDEX idx_derived_words_word
    ON derived_words (word);

CREATE INDEX idx_senses_id_1
    ON senses (id_1);

CREATE INDEX idx_categories_name
    ON categorys (name);


