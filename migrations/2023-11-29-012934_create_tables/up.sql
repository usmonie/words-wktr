-- Your SQL goes here
CREATE TABLE Words
(
    id               SERIAL PRIMARY KEY,
    word             TEXT NOT NULL,
    pos              TEXT NOT NULL,
    lang_code        TEXT NOT NULL,
    lang             TEXT NOT NULL,
    etymology_number INT,
    etymology_text   TEXT
);

CREATE TABLE Related
(
    id        SERIAL PRIMARY KEY,
    roman     TEXT,
    alt       TEXT,
    english   TEXT,
    sense     TEXT,
    word      TEXT NOT NULL,
    taxonomic TEXT,
    qualifier TEXT,
    extra     TEXT
);

CREATE TABLE Urls
(
    url TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE Tags
(
    tag TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE Topics
(
    topic TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE Rubies
(
    ruby TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE Links
(
    link TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE Templates
(
    id        SERIAL PRIMARY KEY,
    word_id   INT NOT NULL,
    args      TEXT,
    name      TEXT,
    expansion TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Translations
(
    id        SERIAL PRIMARY KEY,
    note      TEXT,
    code      TEXT,
    roman     TEXT,
    alt       TEXT,
    english   TEXT,
    sense     TEXT,
    lang      TEXT,
    word      TEXT,
    taxonomic TEXT
);

CREATE TABLE Sounds
(
    id        SERIAL PRIMARY KEY,
    word_id   INT NOT NULL,
    mp3_url   TEXT,
    note      TEXT,
    rhymes    TEXT,
    other     TEXT,
    enpr      TEXT,
    audio_ipa TEXT,
    ogg_url   TEXT,
    form      TEXT,
    ipa       TEXT,
    audio     TEXT,
    text      TEXT,
    homophone TEXT,
    zh_pron   TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Hyphenations
(
    id          SERIAL PRIMARY KEY,
    word_id     INT NOT NULL,
    hyphenation TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Categories
(
    id       SERIAL PRIMARY KEY,
    langcode TEXT,
    orig     TEXT,
    kind     TEXT,
    name     TEXT,
    UNIQUE (name, kind)
);

CREATE TABLE Wikipedia
(
    id             SERIAL PRIMARY KEY,
    word_id        INT NOT NULL,
    wikipedia_link TEXT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Wikidata
(
    id            SERIAL PRIMARY KEY,
    word_id       INT NOT NULL,
    wikidata_link TEXT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Forms
(
    id      SERIAL PRIMARY KEY,
    word_id INT NOT NULL,
    form    TEXT,
    head_nr INT,
    roman   TEXT,
    ipa     TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE InflectionTemplate
(
    id      SERIAL PRIMARY KEY,
    word_id INT  NOT NULL,
    args    TEXT,
    name    TEXT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Instance
(
    id      SERIAL PRIMARY KEY,
    word_id INT NOT NULL,
    sense   TEXT,
    word    TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Descendants
(
    id      SERIAL PRIMARY KEY,
    word_id INT NOT NULL,
    depth   INT,
    text    TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Senses
(
    id          SERIAL PRIMARY KEY,
    word_id     INT NOT NULL,
    sense_id    TEXT,
    head_nr     INT,
    taxonomic   TEXT,
    qualifier   TEXT,
    glosses     TEXT,
    raw_glosses TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Examples
(
    id           SERIAL PRIMARY KEY,
    sense_id     INT NOT NULL,
    note         TEXT,
    example_ref  TEXT,
    roman        TEXT,
    english      TEXT,
    text         TEXT,
    example_type TEXT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id)
);

CREATE TABLE WordFormOfLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordAltOfLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordCategoriesLink
(
    id           SERIAL,
    word_id     INT NOT NULL,
    category_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (category_id) REFERENCES Categories (id),
    PRIMARY KEY (word_id, category_id)
);

CREATE TABLE WordSynonymLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordRelatedLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordDerivedLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordAntonymsLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordHypernymsLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordTroponymsLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordHyponymsLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordMeronymsLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordHolonymsLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordCoordinateTermsLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordProverbsLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE WordCompoundOfLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE TABLE SenseAltOfLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseSynonymLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseRelatedLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseDerivedLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseAntonymsLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseHypernymsLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseTroponymsLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseFormOfLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseHyponymsLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseMeronymsLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseHolonymsLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseCoordinateTermsLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseProverbsLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseCompoundOfLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE TABLE SenseCategoriesLink
(
    id           SERIAL,
    sense_id    INT NOT NULL,
    category_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (category_id) REFERENCES Categories (id),
    PRIMARY KEY (sense_id, category_id)
);

CREATE INDEX idx_word ON Words (word);
CREATE INDEX idx_lang_code ON Words (lang_code);
CREATE INDEX idx_pos ON Words (pos);
-- CREATE INDEX idx_translation_word_id ON Translations (word_id);
CREATE INDEX idx_sound_word_id ON Sounds (word_id);
CREATE INDEX idx_template_word_id ON Templates (word_id);
CREATE INDEX idx_forms_word_id ON Forms (word_id);
CREATE INDEX idx_senses_word_id ON Senses (word_id);
CREATE INDEX idx_instances_word_id ON Instance (word_id);
-- CREATE INDEX idx_forms_ruby_forms_id ON FormsRuby (forms_id);
CREATE INDEX idx_descendants_word_id ON Descendants (word_id);
CREATE INDEX idx_inflection_template_word_id ON InflectionTemplate (word_id);
-- CREATE INDEX idx_instance_tags_instance_id ON InstanceTags (instance_id);
-- CREATE INDEX idx_instance_topics_instance_id ON InstanceTopics (instance_id);
CREATE INDEX idx_wikipedia_word_id ON Wikipedia (word_id);
CREATE INDEX idx_wikidata_word_id ON Wikidata (word_id);
-- CREATE INDEX idx_descendants_tags_descendant_id ON DescendantsTags (descendant_id);
-- CREATE INDEX idx_descendants_templates_descendant_id ON DescendantsTemplates (descendant_id);
-- CREATE INDEX idx_senses_tags_sense_id ON SensesTags (sense_id);
-- CREATE INDEX idx_senses_topics_sense_id ON SensesTopics (sense_id);
CREATE INDEX idx_examples_sense_id ON Examples (sense_id);
-- CREATE INDEX idx_examples_ruby_example_id ON ExamplesRuby (example_id);
-- CREATE INDEX idx_head_templates_word_id ON HeadTemplates (word_id);
-- CREATE INDEX idx_head_templates_template_id ON HeadTemplates (template_id);
CREATE INDEX idx_hyphenation_word_id ON Hyphenations (word_id);
CREATE INDEX idx_categories_word_id_word_id ON WordCategoriesLink (word_id);
CREATE INDEX idx_categories_word_id_category_id ON WordCategoriesLink (word_id);
CREATE INDEX idx_categories_sense_id_sense_id ON SenseCategoriesLink (sense_id);
CREATE INDEX idx_categories_sense_id_category_id ON SenseCategoriesLink (sense_id);

CREATE INDEX idx_word_derived_link_word_id ON WordDerivedLink (word_id);
CREATE INDEX idx_word_derived_link_derived_id ON WordDerivedLink (related_id);
CREATE INDEX idx_word_related_link_word_id ON WordRelatedLink (word_id);
CREATE INDEX idx_word_related_link_related_id ON WordRelatedLink (related_id);
CREATE INDEX idx_word_synonym_link_word_id ON WordSynonymLink (word_id);
CREATE INDEX idx_word_synonym_link_synonym_id ON WordSynonymLink (related_id);
CREATE INDEX idx_word_antonyms_link_word_id ON WordAntonymsLink (word_id);
CREATE INDEX idx_word_antonyms_link_antonyms_id ON WordAntonymsLink (related_id);
CREATE INDEX idx_word_hypernyms_link_word_id ON WordHypernymsLink (word_id);
CREATE INDEX idx_word_hypernyms_link_hypernyms_id ON WordHypernymsLink (related_id);
CREATE INDEX idx_word_troponyms_link_word_id ON WordTroponymsLink (word_id);
CREATE INDEX idx_word_troponyms_link_troponyms_id ON WordTroponymsLink (related_id);
CREATE INDEX idx_word_hyponyms_link_word_id ON WordHyponymsLink (word_id);
CREATE INDEX idx_word_hyponyms_link_hyponyms_id ON WordHyponymsLink (related_id);
CREATE INDEX idx_word_meronyms_link_word_id ON WordMeronymsLink (word_id);
CREATE INDEX idx_word_meronyms_link_meronyms_id ON WordMeronymsLink (related_id);
CREATE INDEX idx_word_holonyms_link_word_id ON WordHolonymsLink (word_id);
CREATE INDEX idx_word_holonyms_link_holonyms_id ON WordHolonymsLink (related_id);
CREATE INDEX idx_word_coordinate_terms_link_word_id ON WordCoordinateTermsLink (word_id);
CREATE INDEX idx_word_coordinate_terms_link_coordinate_terms_id ON WordCoordinateTermsLink (related_id);
CREATE INDEX idx_word_proverbs_link_word_id ON WordProverbsLink (word_id);
CREATE INDEX idx_word_proverbs_link_proverbs_id ON WordProverbsLink (related_id);
CREATE INDEX idx_word_compound_of_link_word_id ON WordCompoundOfLink (word_id);
CREATE INDEX idx_word_compound_of_link_compound_of_id ON WordCompoundOfLink (related_id);

CREATE INDEX idx_sense_derived_link_sense_id ON SenseSynonymLink (sense_id);
CREATE INDEX idx_sense_derived_link_derived_id ON SenseSynonymLink (related_id);
CREATE INDEX idx_sense_related_link_sense_id ON SenseRelatedLink (sense_id);
CREATE INDEX idx_sense_related_link_related_id ON SenseRelatedLink (related_id);
CREATE INDEX idx_sense_synonym_link_sense_id ON SenseDerivedLink (sense_id);
CREATE INDEX idx_sense_synonym_link_synonym_id ON SenseDerivedLink (related_id);
CREATE INDEX idx_sense_antonyms_link_sense_id ON SenseAntonymsLink (sense_id);
CREATE INDEX idx_sense_antonyms_link_antonyms_id ON SenseAntonymsLink (related_id);
CREATE INDEX idx_sense_hypernyms_link_sense_id ON SenseHypernymsLink (sense_id);
CREATE INDEX idx_sense_hypernyms_link_hypernyms_id ON SenseHypernymsLink (related_id);
CREATE INDEX idx_sense_troponyms_link_sense_id ON SenseTroponymsLink (sense_id);
CREATE INDEX idx_sense_troponyms_link_troponyms_id ON SenseTroponymsLink (related_id);
CREATE INDEX idx_sense_hyponyms_link_sense_id ON SenseHyponymsLink (sense_id);
CREATE INDEX idx_sense_hyponyms_link_hyponyms_id ON SenseHyponymsLink (related_id);
CREATE INDEX idx_sense_holonyms_link_sense_id ON SenseHolonymsLink (sense_id);
CREATE INDEX idx_sense_holonyms_link_holonyms_id ON SenseHolonymsLink (related_id);
CREATE INDEX idx_sense_meronyms_link_sense_id ON SenseMeronymsLink (sense_id);
CREATE INDEX idx_sense_meronyms_link_meronyms_id ON SenseMeronymsLink (related_id);
CREATE INDEX idx_sense_coordinate_terms_link_sense_id ON SenseCoordinateTermsLink (sense_id);
CREATE INDEX idx_sense_coordinate_terms_link_coordinate_terms_id ON SenseCoordinateTermsLink (related_id);
CREATE INDEX idx_sense_proverbs_link_sense_id ON SenseProverbsLink (sense_id);
CREATE INDEX idx_sense_proverbs_link_proverbs_id ON SenseProverbsLink (related_id);
CREATE INDEX idx_sense_compound_of_link_sense_id ON SenseCompoundofLink (sense_id);
CREATE INDEX idx_sense_compound_of_link_compound_of_id ON SenseCompoundofLink (related_id);

CREATE TABLE WordAbbreviationsLink
(
    id           SERIAL,
    word_id    INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (word_id, related_id)
);

CREATE INDEX idx_word_abbreviations_link_word_id ON WordAbbreviationsLink (word_id);
CREATE INDEX idx_word_abbreviations_link_abbreviations_id ON WordAbbreviationsLink (related_id);

CREATE TABLE SenseAbbreviationsLink
(
    id           SERIAL,
    sense_id   INT NOT NULL,
    related_id INT NOT NULL,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    PRIMARY KEY (sense_id, related_id)
);

CREATE INDEX idx_sense_abbreviations_link_sense_id ON SenseAbbreviationsLink (sense_id);
CREATE INDEX idx_sense_abbreviations_link_abbreviations_id ON SenseAbbreviationsLink (related_id);

-- Создание таблиц связей many-to-many
CREATE TABLE related_tags
(
    id           SERIAL,
    related_id INT,
    tag        TEXT,
    PRIMARY KEY (related_id, tag),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    FOREIGN KEY (tag) REFERENCES Tags (tag)
);

CREATE TABLE related_urls
(
    id           SERIAL,
    related_id INT,
    url        TEXT,
    PRIMARY KEY (related_id, url),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    FOREIGN KEY (url) REFERENCES Urls (url)
);

CREATE TABLE related_topics
(
    id           SERIAL,
    related_id INT,
    topic      TEXT,
    PRIMARY KEY (related_id, topic),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    FOREIGN KEY (topic) REFERENCES Topics (topic)
);

CREATE TABLE related_ruby
(
    id           SERIAL,
    related_id INT,
    ruby       TEXT,
    PRIMARY KEY (related_id, ruby),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    FOREIGN KEY (ruby) REFERENCES Rubies (ruby)
);

CREATE TABLE forms_tags
(
    id           SERIAL,
    forms_id INT,
    tag      TEXT,
    PRIMARY KEY (forms_id, tag),
    FOREIGN KEY (forms_id) REFERENCES Forms (id),
    FOREIGN KEY (tag) REFERENCES Tags (tag)
);

CREATE TABLE forms_ruby
(
    id           SERIAL,
    forms_id INT,
    ruby     TEXT,
    PRIMARY KEY (forms_id, ruby),
    FOREIGN KEY (forms_id) REFERENCES Forms (id),
    FOREIGN KEY (ruby) REFERENCES Rubies (ruby)
);

CREATE TABLE translation_tags
(
    id           SERIAL,
    translation_id INT,
    tag            TEXT,
    PRIMARY KEY (translation_id, tag),
    FOREIGN KEY (translation_id) REFERENCES Translations (id),
    FOREIGN KEY (tag) REFERENCES Tags (tag)
);

CREATE TABLE translation_topics
(
    id           SERIAL,
    translation_id INT,
    topic          TEXT,
    PRIMARY KEY (translation_id, topic),
    FOREIGN KEY (translation_id) REFERENCES Translations (id),
    FOREIGN KEY (topic) REFERENCES Topics (topic)
);

-- Создание индексов
CREATE INDEX idx_related_tags_related_id ON related_tags (related_id);
CREATE INDEX idx_related_tags_tag ON related_tags (tag);

CREATE INDEX idx_related_urls_related_id ON related_urls (related_id);
CREATE INDEX idx_related_urls_url ON related_urls (url);

CREATE INDEX idx_related_topics_related_id ON related_topics (related_id);
CREATE INDEX idx_related_topics_topic ON related_topics (topic);

CREATE TABLE instance_tags
(
    id           SERIAL,
    instance_id INT NOT NULL,
    tag         TEXT,
    PRIMARY KEY (instance_id, tag),
    FOREIGN KEY (instance_id) REFERENCES Instance (id),
    FOREIGN KEY (tag) REFERENCES Tags (tag)
);

CREATE TABLE instance_topics
(
    id           SERIAL,
    instance_id INT NOT NULL,
    topic       TEXT NOT NULL,
    PRIMARY KEY (instance_id, topic),
    FOREIGN KEY (instance_id) REFERENCES Instance (id),
    FOREIGN KEY (topic) REFERENCES Topics (topic)
);

CREATE TABLE sound_tags
(
    id           SERIAL,
    sound_id INT NOT NULL,
    tag      TEXT NOT NULL,
    PRIMARY KEY (sound_id, tag),
    FOREIGN KEY (sound_id) REFERENCES Sounds (id),
    FOREIGN KEY (tag) REFERENCES Tags (tag)
);

CREATE TABLE sound_topics
(
    id           SERIAL,
    sound_id INT NOT NULL,
    topic    TEXT NOT NULL,
    PRIMARY KEY (sound_id, topic),
    FOREIGN KEY (sound_id) REFERENCES Sounds (id),
    FOREIGN KEY (topic) REFERENCES Topics (topic)
);

CREATE TABLE example_ruby
(
    id           SERIAL,
    example_id INT NOT NULL,
    ruby       TEXT NOT NULL,
    PRIMARY KEY (example_id, ruby),
    FOREIGN KEY (example_id) REFERENCES Examples (id),
    FOREIGN KEY (ruby) REFERENCES Rubies (ruby)
);

CREATE TABLE descendants_tags
(
    id           SERIAL,
    descendant_id INT NOT NULL,
    tag           TEXT NOT NULL,
    PRIMARY KEY (descendant_id, tag),
    FOREIGN KEY (descendant_id) REFERENCES Descendants (id),
    FOREIGN KEY (tag) REFERENCES Tags (tag)
);

CREATE TABLE descendants_templates
(
    id           SERIAL,
    descendant_id INT NOT NULL,
    template_id   INT NOT NULL,
    PRIMARY KEY (descendant_id, template_id),
    FOREIGN KEY (descendant_id) REFERENCES Descendants (id),
    FOREIGN KEY (template_id) REFERENCES Templates (id)
);

-- CREATE TABLE head_templates
-- (
--     head_id     INT NOT NULL,
--     template_id INT NOT NULL,
--     PRIMARY KEY (head_id, template_id),
--     FOREIGN KEY (head_id) REFERENCES Head (id),
--     FOREIGN KEY (template_id) REFERENCES Templates (id)
-- );

CREATE TABLE sense_tags
(
    id           SERIAL,
    sense_id INT NOT NULL,
    tag      TEXT NOT NULL,
    PRIMARY KEY (sense_id, tag),
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (tag) REFERENCES Tags (tag)
);

CREATE TABLE sense_topics
(
    id           SERIAL,
    sense_id INT NOT NULL,
    topic    TEXT NOT NULL,
    PRIMARY KEY (sense_id, topic),
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (topic) REFERENCES Topics (topic)
);

CREATE TABLE sense_links
(
    id           SERIAL,
    sense_id INT NOT NULL,
    link     TEXT NOT NULL,
    PRIMARY KEY (sense_id, link),
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (link) REFERENCES Links (link)
);

CREATE INDEX idx_instance_tags_instance_id ON instance_tags (instance_id);
CREATE INDEX idx_instance_tags_tag ON instance_tags (tag);

CREATE INDEX idx_instance_topics_instance_id ON instance_topics (instance_id);
CREATE INDEX idx_instance_topics_topic ON instance_topics (topic);

CREATE INDEX idx_sound_tags_sound_id ON sound_tags (sound_id);
CREATE INDEX idx_sound_tags_tag ON sound_tags (tag);

CREATE INDEX idx_sound_topics_sound_id ON sound_topics (sound_id);
CREATE INDEX idx_sound_topics_topic ON sound_topics (topic);

CREATE TABLE translations_words_link
(
    id           SERIAL,
    translation_id INTEGER NOT NULL,
    word_id INTEGER NOT NULL,
    PRIMARY KEY (translation_id, word_id),
    FOREIGN KEY (translation_id) REFERENCES translations (id),
    FOREIGN KEY (word_id) REFERENCES words (id)
);

CREATE INDEX idx_translations_words_link_translation_id ON translations_words_link (translation_id);
CREATE INDEX idx_translations_words_link_word_id ON translations_words_link (word_id);