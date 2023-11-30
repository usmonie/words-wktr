CREATE TABLE hypernyms
(
    id       SERIAL PRIMARY KEY,
    word_id  INTEGER, -- Ссылка на таблицу слов
    topic_id INTEGER, -- Ссылка на таблицу тем
    word     TEXT,
    sense    TEXT,
    source   TEXT,
    english  TEXT,
    tags     TEXT,
    alt      TEXT,
    roman    TEXT,
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE coordinate_terms
(
    id       SERIAL PRIMARY KEY,
    word_id  INTEGER, -- Ссылка на таблицу слов
    alt      TEXT,
    sense    TEXT,
    word     TEXT,
    english  TEXT,
    topic_id INTEGER, -- Ссылка на таблицу тем
    urls     TEXT,    -- Может быть разделено запятыми или другим разделителем
    source   TEXT,
    roman    TEXT,
    tags     TEXT,
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE descendants_templates_args
(
    id          SERIAL PRIMARY KEY,
    template_id INTEGER, -- Ссылка на таблицу шаблонов потомков
    arg_key     TEXT,
    arg_value   TEXT,
    FOREIGN KEY (template_id) REFERENCES descendants (id)
);

CREATE TABLE descendants
(
    id          SERIAL PRIMARY KEY,
    word_id     INTEGER, -- Ссылка на таблицу слов
    text        TEXT,
    template_id INTEGER, -- Ссылка на таблицу шаблонов
    depth       INTEGER,
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (template_id) REFERENCES templates (id)
);

CREATE TABLE sounds
(
    id        SERIAL PRIMARY KEY,
    word_id   INTEGER, -- Ссылка на таблицу слов
    topic_id  INTEGER, -- Ссылка на таблицу тем
    mp3_url   TEXT,
    text      TEXT,
    enpr      TEXT,
    ipa       TEXT,
    rhymes    TEXT,
    other     TEXT,
    homophone TEXT,
    tags      VARCHAR(255),
    audio_ipa TEXT,
    audio     TEXT,
    note      TEXT,
    ogg_url   TEXT,
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE related
(
    id           SERIAL PRIMARY KEY,
    word_id      INTEGER, -- Ссылка на таблицу слов
    ruby         TEXT,
    roman        VARCHAR(255),
    topic_id     INTEGER, -- Ссылка на таблицу тем
    tags         VARCHAR(255),
    source       VARCHAR(255),
    related_word VARCHAR(255),
    sense        VARCHAR(255),
    english      VARCHAR(255),
    urls         TEXT,
    alt          VARCHAR(255),
    taxonomic    VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE holonyms
(
    id           SERIAL PRIMARY KEY,
    word_id      INTEGER, -- Ссылка на таблицу слов
    english      VARCHAR(255),
    sense        VARCHAR(255),
    holonym_word VARCHAR(255),
    topic_id     INTEGER, -- Ссылка на таблицу тем
    source       VARCHAR(255),
    tags         VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE abbreviations
(
    id           SERIAL PRIMARY KEY,
    word_id      INTEGER, -- Ссылка на таблицу слов
    alt          VARCHAR(255),
    topic_id     INTEGER, -- Ссылка на таблицу тем
    abbreviation VARCHAR(255),
    sense        VARCHAR(255),
    urls         TEXT,
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_derived
(
    id            SERIAL PRIMARY KEY,
    sense_id      INTEGER, -- Ссылка на таблицу смыслов слова
    source        VARCHAR(255),
    derived_sense VARCHAR(255),
    roman         VARCHAR(255),
    english       VARCHAR(255),
    urls          TEXT,
    tags          VARCHAR(255),
    alt           VARCHAR(255),
    taxonomic     VARCHAR(255),
    derived_word  VARCHAR(255),
    topic_id      INTEGER, -- Ссылка на таблицу тем
    FOREIGN KEY (sense_id) REFERENCES senses (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_compound_of
(
    id            SERIAL PRIMARY KEY,
    sense_id      INTEGER, -- Ссылка на таблицу смыслов слова
    extra         TEXT,
    compound_word VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_antonyms
(
    id       SERIAL PRIMARY KEY,
    sense_id INTEGER, -- Ссылка на таблицу смыслов слова
    alt      VARCHAR(255),
    topic_id INTEGER, -- Ссылка на таблицу тем
    antonym  VARCHAR(255),
    english  VARCHAR(255),
    tags     VARCHAR(255),
    source   VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_glosses
(
    id       SERIAL PRIMARY KEY,
    sense_id INTEGER, -- Ссылка на таблицу смыслов слова
    gloss    TEXT,
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_synonyms
(
    id        SERIAL PRIMARY KEY,
    sense_id  INTEGER, -- Ссылка на таблицу смыслов слова
    synonym   VARCHAR(255),
    english   VARCHAR(255),
    tags      VARCHAR(255),
    extra     TEXT,
    topic_id  INTEGER, -- Ссылка на таблицу тем
    alt       VARCHAR(255),
    roman     VARCHAR(255),
    taxonomic VARCHAR(255),
    qualifier VARCHAR(255),
    urls      TEXT,
    source    VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_hyponyms
(
    id        SERIAL PRIMARY KEY,
    sense_id  INTEGER, -- Ссылка на таблицу смыслов слова
    hyponym   VARCHAR(255),
    sense     VARCHAR(255),
    alt       VARCHAR(255),
    source    VARCHAR(255),
    topic_id  INTEGER, -- Ссылка на таблицу тем
    qualifier VARCHAR(255),
    tags      VARCHAR(255),
    english   VARCHAR(255),
    taxonomic VARCHAR(255),
    urls      TEXT,
    roman     VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_hyponyms
(
    id        SERIAL PRIMARY KEY,
    sense_id  INTEGER, -- Ссылка на таблицу смыслов слова
    hyponym   VARCHAR(255),
    sense     VARCHAR(255),
    alt       VARCHAR(255),
    source    VARCHAR(255),
    topic_id  INTEGER, -- Ссылка на таблицу тем
    qualifier VARCHAR(255),
    tags      VARCHAR(255),
    english   VARCHAR(255),
    taxonomic VARCHAR(255),
    urls      TEXT,
    roman     VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_related
(
    id            SERIAL PRIMARY KEY,
    sense_id      INTEGER, -- Ссылка на таблицу смыслов слова
    related_sense VARCHAR(255),
    source        VARCHAR(255),
    urls          TEXT,
    tags          VARCHAR(255),
    taxonomic     VARCHAR(255),
    alt           VARCHAR(255),
    english       VARCHAR(255),
    qualifier     VARCHAR(255),
    roman         VARCHAR(255),
    related_word  VARCHAR(255),
    topic_id      INTEGER, -- Ссылка на таблицу тем
    FOREIGN KEY (sense_id) REFERENCES senses (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_categories
(
    id       SERIAL PRIMARY KEY,
    sense_id INTEGER, -- Ссылка на таблицу смыслов слова
    _dis     VARCHAR(255),
    langcode VARCHAR(255),
    name     VARCHAR(255),
    orig     VARCHAR(255),
    kind     VARCHAR(255),
    parents  TEXT,    -- Может быть списком, разделенным запятыми
    source   VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_holonyms
(
    id        SERIAL PRIMARY KEY,
    sense_id  INTEGER, -- Ссылка на таблицу смыслов слова
    holonym   VARCHAR(255),
    sense     VARCHAR(255),
    alt       VARCHAR(255),
    source    VARCHAR(255),
    topic_id  INTEGER, -- Ссылка на таблицу тем
    qualifier VARCHAR(255),
    tags      VARCHAR(255),
    english   VARCHAR(255),
    taxonomic VARCHAR(255),
    urls      TEXT,
    roman     VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_head_nr
(
    id       SERIAL PRIMARY KEY,
    sense_id INTEGER, -- Ссылка на таблицу смыслов слова
    head_nr  INTEGER,
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_alt_of
(
    id          SERIAL PRIMARY KEY,
    sense_id    INTEGER, -- Ссылка на таблицу смыслов слова
    alt_of_word VARCHAR(255),
    extra       TEXT,
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_coordinate_terms
(
    id              SERIAL PRIMARY KEY,
    sense_id        INTEGER, -- Ссылка на таблицу смыслов слова
    coordinate_term VARCHAR(255),
    sense           VARCHAR(255),
    alt             VARCHAR(255),
    source          VARCHAR(255),
    topic_id        INTEGER, -- Ссылка на таблицу тем
    tags            VARCHAR(255),
    english         VARCHAR(255),
    taxonomic       VARCHAR(255),
    urls            TEXT,
    roman           VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_examples
(
    id           SERIAL PRIMARY KEY,
    sense_id     INTEGER, -- Ссылка на таблицу смыслов слова
    example_type VARCHAR(255),
    roman        VARCHAR(255),
    ref          TEXT,
    example_text TEXT,
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_qualifier
(
    id        SERIAL PRIMARY KEY,
    sense_id  INTEGER, -- Ссылка на таблицу смыслов слова
    qualifier TEXT,
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_tags
(
    id       SERIAL PRIMARY KEY,
    sense_id INTEGER, -- Ссылка на таблицу смыслов слова
    tag      VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_form_of
(
    id           SERIAL PRIMARY KEY,
    sense_id     INTEGER, -- Ссылка на таблицу смыслов слова
    form_of_word VARCHAR(255),
    extra        TEXT,
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_instances
(
    id            SERIAL PRIMARY KEY,
    sense_id      INTEGER, -- Ссылка на таблицу смыслов слова
    instance_word VARCHAR(255),
    tags          VARCHAR(255),
    topics        TEXT,    -- Может быть списком, разделенным запятыми
    source        VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_meronyms
(
    id        SERIAL PRIMARY KEY,
    sense_id  INTEGER, -- Ссылка на таблицу смыслов слова
    meronym   VARCHAR(255),
    sense     VARCHAR(255),
    alt       VARCHAR(255),
    source    VARCHAR(255),
    topic_id  INTEGER, -- Ссылка на таблицу тем
    tags      VARCHAR(255),
    english   VARCHAR(255),
    taxonomic VARCHAR(255),
    urls      TEXT,
    roman     VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE senses_troponyms
(
    id       SERIAL PRIMARY KEY,
    sense_id INTEGER, -- Ссылка на таблицу смыслов слова
    troponym VARCHAR(255),
    sense    VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_wikipedia
(
    id            SERIAL PRIMARY KEY,
    sense_id      INTEGER, -- Ссылка на таблицу смыслов слова
    wikipedia_url TEXT,
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_links
(
    id       SERIAL PRIMARY KEY,
    sense_id INTEGER, -- Ссылка на таблицу смыслов слова
    link     TEXT,
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_translations
(
    id                SERIAL PRIMARY KEY,
    sense_id          INTEGER, -- Ссылка на таблицу смыслов слова
    note              TEXT,
    code              VARCHAR(255),
    roman             VARCHAR(255),
    alt               VARCHAR(255),
    translation_sense VARCHAR(255),
    tags              VARCHAR(255),
    translation_word  VARCHAR(255),
    topics            TEXT,    -- Может быть списком, разделенным запятыми
    taxonomic         VARCHAR(255),
    english           VARCHAR(255),
    lang              VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_topics
(
    id       SERIAL PRIMARY KEY,
    sense_id INTEGER, -- Ссылка на таблицу смыслов слова
    topic    VARCHAR(255),
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE senses_wikidata
(
    id           SERIAL PRIMARY KEY,
    sense_id     INTEGER, -- Ссылка на таблицу смыслов слова
    wikidata_url TEXT,
    FOREIGN KEY (sense_id) REFERENCES senses (id)
);

CREATE TABLE hyponyms
(
    id        SERIAL PRIMARY KEY,
    word_id   INTEGER, -- Ссылка на таблицу слов
    hyponym   VARCHAR(255),
    sense     VARCHAR(255),
    alt       VARCHAR(255),
    source    VARCHAR(255),
    topic_id  INTEGER, -- Ссылка на таблицу тем
    tags      VARCHAR(255),
    english   VARCHAR(255),
    taxonomic VARCHAR(255),
    urls      TEXT,
    roman     VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE synonyms
(
    id        SERIAL PRIMARY KEY,
    word_id   INTEGER, -- Ссылка на таблицу слов
    synonym   VARCHAR(255),
    source    VARCHAR(255),
    alt       VARCHAR(255),
    taxonomic VARCHAR(255),
    topics    TEXT,
    roman     VARCHAR(255),
    english   VARCHAR(255),
    sense     VARCHAR(255),
    urls      TEXT,
    tags      VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE antonyms
(
    id      SERIAL PRIMARY KEY,
    word_id INTEGER, -- Ссылка на таблицу слов
    antonym VARCHAR(255),
    english VARCHAR(255),
    source  VARCHAR(255),
    sense   VARCHAR(255),
    tags    VARCHAR(255),
    roman   VARCHAR(255),
    topics  TEXT,
    alt     VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE hyphenation
(
    id              SERIAL PRIMARY KEY,
    word_id         INTEGER, -- Ссылка на таблицу слов
    hyphenated_word VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE forms
(
    id      SERIAL PRIMARY KEY,
    word_id INTEGER, -- Ссылка на таблицу слов
    form    VARCHAR(255),
    tags    VARCHAR(255),
    topics  TEXT,    -- Может быть списком, разделенным запятыми
    head_nr INTEGER,
    source  VARCHAR(255),
    roman   VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE meronyms
(
    id        SERIAL PRIMARY KEY,
    word_id   INTEGER, -- Ссылка на таблицу слов
    meronym   VARCHAR(255),
    sense     VARCHAR(255),
    alt       VARCHAR(255),
    source    VARCHAR(255),
    topic_id  INTEGER, -- Ссылка на таблицу тем
    tags      VARCHAR(255),
    english   VARCHAR(255),
    taxonomic VARCHAR(255),
    urls      TEXT,
    roman     VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE original_title
(
    id             SERIAL PRIMARY KEY,
    word_id        INTEGER, -- Ссылка на таблицу слов
    original_title VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE translations
(
    id               SERIAL PRIMARY KEY,
    word_id          INTEGER, -- Ссылка на таблицу слов
    roman            VARCHAR(255),
    taxonomic        VARCHAR(255),
    tags             VARCHAR(255),
    note             TEXT,
    lang             VARCHAR(255),
    alt              VARCHAR(255),
    translation_word VARCHAR(255),
    topics           TEXT,    -- Может быть списком, разделенным запятыми
    code             VARCHAR(255),
    sense            VARCHAR(255),
    english          VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE inflection_templates
(
    id            SERIAL PRIMARY KEY,
    word_id       INTEGER, -- Ссылка на таблицу слов
    template_name VARCHAR(255),
    args          TEXT,    -- Может содержать сериализованные аргументы шаблона
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE form_of
(
    id           SERIAL PRIMARY KEY,
    word_id      INTEGER, -- Ссылка на таблицу слов
    form_of_word VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE head_templates
(
    id            SERIAL PRIMARY KEY,
    word_id       INTEGER, -- Ссылка на таблицу слов
    template_name VARCHAR(255),
    args          TEXT,    -- Может содержать сериализованные аргументы шаблона
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE etymology_templates
(
    id            SERIAL PRIMARY KEY,
    word_id       INTEGER, -- Ссылка на таблицу слов
    template_name VARCHAR(255),
    args          TEXT,    -- Может содержать сериализованные аргументы шаблона
    FOREIGN KEY (word_id) REFERENCES words (id)
);


CREATE TABLE troponyms
(
    id       SERIAL PRIMARY KEY,
    word_id  INTEGER, -- Ссылка на таблицу слов
    troponym VARCHAR(255),
    sense    VARCHAR(255),
    topic_id INTEGER, -- Ссылка на таблицу тем
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE derived
(
    id           SERIAL PRIMARY KEY,
    word_id      INTEGER, -- Ссылка на таблицу слов
    derived_word VARCHAR(255),
    sense        VARCHAR(255),
    roman        VARCHAR(255),
    english      VARCHAR(255),
    urls         TEXT,
    tags         VARCHAR(255),
    alt          VARCHAR(255),
    taxonomic    VARCHAR(255),
    qualifier    VARCHAR(255),
    topic_id     INTEGER, -- Ссылка на таблицу тем
    FOREIGN KEY (word_id) REFERENCES words (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE instances
(
    id       SERIAL PRIMARY KEY,
    word_id  INTEGER, -- Ссылка на таблицу слов
    instance VARCHAR(255),
    sense    VARCHAR(255),
    source   VARCHAR(255),
    tags     VARCHAR(255),
    topics   TEXT,    -- Может быть списком, разделенным запятыми
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE categories
(
    id       SERIAL PRIMARY KEY,
    word_id  INTEGER, -- Ссылка на таблицу слов
    name     VARCHAR(255),
    orig     VARCHAR(255),
    kind     VARCHAR(255),
    source   VARCHAR(255),
    _dis     VARCHAR(255),
    parents  TEXT,    -- Может быть списком, разделенным запятыми
    langcode VARCHAR(255),
    FOREIGN KEY (word_id) REFERENCES words (id)
);
CREATE TABLE head_templates_args
(
    id          SERIAL PRIMARY KEY,
    template_id INTEGER, -- Ссылка на таблицу head_templates
    arg_key     VARCHAR(255),
    arg_value   TEXT,
    FOREIGN KEY (template_id) REFERENCES head_templates (id)
);

CREATE TABLE wikidata
(
    id           SERIAL PRIMARY KEY,
    word_id      INTEGER, -- Ссылка на таблицу слов
    wikidata_url TEXT,
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE instances_tags
(
    id          SERIAL PRIMARY KEY,
    instance_id INTEGER, -- Ссылка на таблицу instances
    tag         VARCHAR(255),
    FOREIGN KEY (instance_id) REFERENCES instances (id)
);

CREATE TABLE instances_topics
(
    id          SERIAL PRIMARY KEY,
    instance_id INTEGER, -- Ссылка на таблицу instances
    topic       VARCHAR(255),
    FOREIGN KEY (instance_id) REFERENCES instances (id)
);

CREATE TABLE words
(
    id             SERIAL PRIMARY KEY,
    word           VARCHAR(255) NOT NULL,
    lang_code      VARCHAR(10),
    pos            VARCHAR(100), -- Part of speech
    etymology_text TEXT,
    pronunciation  TEXT,
    hyphenation    TEXT,
    original_title VARCHAR(255),
    wikipedia_link TEXT,
    wikidata_link  TEXT
);

CREATE TABLE sounds_topics
(
    id       SERIAL PRIMARY KEY,
    sound_id INTEGER, -- Ссылка на таблицу sounds
    topic_id INTEGER, -- Ссылка на таблицу topics
    FOREIGN KEY (sound_id) REFERENCES sounds (id),
    FOREIGN KEY (topic_id) REFERENCES topics (id)
);

CREATE TABLE topics
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    description TEXT
);

CREATE TABLE sounds
(
    id        SERIAL PRIMARY KEY,
    word_id   INTEGER, -- Ссылка на таблицу слов
    mp3_url   TEXT,
    ogg_url   TEXT,
    ipa       TEXT,
    enpr      TEXT,
    rhymes    TEXT,
    audio_ipa TEXT,
    audio     TEXT,
    note      TEXT,
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE sounds_other
(
    id               SERIAL PRIMARY KEY,
    sound_id         INTEGER, -- Ссылка на таблицу sounds
    other_sound_info TEXT,
    FOREIGN KEY (sound_id) REFERENCES sounds (id)
);

CREATE TABLE sounds_homophone
(
    id        SERIAL PRIMARY KEY,
    sound_id  INTEGER, -- Ссылка на таблицу sounds
    homophone VARCHAR(255),
    FOREIGN KEY (sound_id) REFERENCES sounds (id)
);

CREATE TABLE senses
(
    id             SERIAL PRIMARY KEY,
    word_id        INTEGER, -- Ссылка на таблицу слов
    definition     TEXT,
    part_of_speech VARCHAR(255),
    etymology      TEXT,
    pronunciation  TEXT,
    hyphenation    TEXT,
    usage_notes    TEXT,
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE TABLE templates
(
    id            SERIAL PRIMARY KEY,
    name          VARCHAR(255) NOT NULL,
    description   TEXT,
    template_type VARCHAR(100),
    content       TEXT
);

CREATE INDEX idx_word ON words (word);
CREATE INDEX idx_senses_word_id ON senses (word_id);
CREATE INDEX idx_hypernyms_word_id ON hypernyms (word_id);
CREATE INDEX idx_antonyms_word_id ON antonyms (word_id);
CREATE INDEX idx_synonyms_word_id ON synonyms (word_id);
CREATE INDEX idx_forms_word_id ON forms (word_id);
CREATE INDEX idx_sounds_word_id ON sounds (word_id);
CREATE INDEX idx_senses_part_of_speech ON senses (part_of_speech);
CREATE INDEX idx_forms_tags ON forms (tags);


