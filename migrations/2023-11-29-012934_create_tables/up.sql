-- Your SQL goes here
CREATE TABLE Words
(
    id               SERIAL PRIMARY KEY,
    word             TEXT NOT NULL,
    pos              TEXT NOT NULL,
    source           TEXT,
    lang_code        TEXT NOT NULL,
    lang             TEXT NOT NULL,
    original_title   TEXT,
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
    word      TEXT,
    source    TEXT,
    taxonomic TEXT,
    qualifier TEXT,
    extra     TEXT
);

CREATE TABLE RelatedUrls
(
    id         SERIAL PRIMARY KEY,
    related_id INT,
    url        TEXT,
    FOREIGN KEY (related_id) REFERENCES Related (id)
);

CREATE TABLE RelatedTopics
(
    id         SERIAL PRIMARY KEY,
    related_id INT,
    topic      TEXT,
    FOREIGN KEY (related_id) REFERENCES Related (id)
);

CREATE TABLE RelatedTags
(
    id         SERIAL PRIMARY KEY,
    related_id INT,
    tag        TEXT,
    FOREIGN KEY (related_id) REFERENCES Related (id)
);

CREATE TABLE RelatedRuby
(
    id         SERIAL PRIMARY KEY,
    related_id INT,
    ruby       TEXT,
    FOREIGN KEY (related_id) REFERENCES Related (id)
);

CREATE TABLE Templates
(
    id        SERIAL PRIMARY KEY,
    word_id   INT,
    args      TEXT,
    name      TEXT,
    expansion TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Translations
(
    id        SERIAL PRIMARY KEY,
    word_id   INT,
    note      TEXT,
    code      TEXT,
    roman     TEXT,
    alt       TEXT,
    english   TEXT,
    sense     TEXT,
    lang      TEXT,
    word      TEXT,
    taxonomic TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE TranslationTopics
(
    id             SERIAL PRIMARY KEY,
    translation_id INT,
    topic          TEXT,
    FOREIGN KEY (translation_id) REFERENCES Translations (id)
);

CREATE TABLE TranslationTags
(
    id             SERIAL PRIMARY KEY,
    translation_id INT,
    tag            TEXT,
    FOREIGN KEY (translation_id) REFERENCES Translations (id)
);

CREATE TABLE Sounds
(
    id        SERIAL PRIMARY KEY,
    word_id   INT,
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
    word_id     INT,
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
    source   TEXT
);

CREATE TABLE Wikipedia
(
    id             SERIAL PRIMARY KEY,
    word_id        INT,
    wikipedia_link TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Wikidata
(
    id            SERIAL PRIMARY KEY,
    word_id       INT,
    wikidata_link TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Topics
(
    id      SERIAL PRIMARY KEY,
    word_id INT,
    topic   TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Forms
(
    id      SERIAL PRIMARY KEY,
    word_id INT,
    form    TEXT,
    head_nr INT,
    roman   TEXT,
    source  TEXT,
    ipa     TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE FormsRuby
(
    id       SERIAL PRIMARY KEY,
    forms_id INT,
    ruby     TEXT,
    FOREIGN KEY (forms_id) REFERENCES Forms (id)
);

CREATE TABLE FormsTags
(
    id       SERIAL PRIMARY KEY,
    forms_id INT,
    tag      TEXT,
    FOREIGN KEY (forms_id) REFERENCES Forms (id)
);

CREATE TABLE InflectionTemplate
(
    id      SERIAL PRIMARY KEY,
    word_id INT,
    args    TEXT,
    name    TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE Instance
(
    id      SERIAL PRIMARY KEY,
    word_id INT,
    sense   TEXT,
    source  TEXT,
    word    TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE InstanceTags
(
    id          SERIAL PRIMARY KEY,
    instance_id INT,
    tag         TEXT,
    FOREIGN KEY (instance_id) REFERENCES Instance (id)
);

CREATE TABLE InstanceTopics
(
    id          SERIAL PRIMARY KEY,
    instance_id INT,
    topic       TEXT,
    FOREIGN KEY (instance_id) REFERENCES Instance (id)
);

CREATE TABLE SoundTopics
(
    id       SERIAL PRIMARY KEY,
    sound_id INT,
    topic    TEXT,
    FOREIGN KEY (sound_id) REFERENCES Sounds (id)
);

CREATE TABLE SoundTags
(
    id       SERIAL PRIMARY KEY,
    sound_id INT,
    tag      TEXT,
    FOREIGN KEY (sound_id) REFERENCES Sounds (id)
);

CREATE TABLE Descendants
(
    id      SERIAL PRIMARY KEY,
    word_id INT,
    depth   INT,
    text    TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE DescendantsTags
(
    id            SERIAL PRIMARY KEY,
    descendant_id INT,
    tag           TEXT,
    FOREIGN KEY (descendant_id) REFERENCES Descendants (id)
);

CREATE TABLE DescendantsTemplates
(
    id            SERIAL PRIMARY KEY,
    descendant_id INT,
    template_id   INT,
    FOREIGN KEY (descendant_id) REFERENCES Descendants (id),
    FOREIGN KEY (template_id) REFERENCES Templates (id)
);

CREATE TABLE HeadTemplates
(
    id          SERIAL PRIMARY KEY,
    word_id     INT,
    template_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (template_id) REFERENCES Templates (id)
);

CREATE TABLE Senses
(
    id        SERIAL PRIMARY KEY,
    word_id   INT,
    sense_id  TEXT,
    head_nr   INT,
    taxonomic TEXT,
    qualifier TEXT,
    glosses          TEXT,
    raw_glosses      TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE SensesTags
(
    id       SERIAL PRIMARY KEY,
    sense_id INT,
    tag      TEXT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id)
);

CREATE TABLE SensesTopics
(
    id       SERIAL PRIMARY KEY,
    sense_id INT,
    topic    TEXT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id)
);

CREATE TABLE SensesLinks
(
    id       SERIAL PRIMARY KEY,
    sense_id INT,
    links    TEXT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id)
);

CREATE TABLE Examples
(
    id           SERIAL PRIMARY KEY,
    sense_id     INT,
    note         TEXT,
    example_ref  TEXT,
    roman        TEXT,
    english      TEXT,
    text         TEXT,
    example_type TEXT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id)
);

CREATE TABLE ExamplesRuby
(
    id         SERIAL PRIMARY KEY,
    example_id INT,
    ruby       TEXT,
    FOREIGN KEY (example_id) REFERENCES Examples (id)
);

CREATE TABLE GenericVectorField
(
    id          SERIAL PRIMARY KEY,
    word_id     INT,
    field_value TEXT,
    FOREIGN KEY (word_id) REFERENCES Words (id)
);

CREATE TABLE WordFormOfLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordAltOfLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordCategoriesLink
(
    id         SERIAL PRIMARY KEY,
    word_id   INT,
    category_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (category_id) REFERENCES Categories (id),
    UNIQUE (word_id, category_id)
);

CREATE TABLE WordSynonymLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordRelatedLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordDerivedLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordAntonymsLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordHypernymsLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordTroponymsLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordHyponymsLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordMeronymsLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordHolonymsLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordCoordinateTermsLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordProverbsLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE WordCompoundOfLink
(
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE TABLE SenseAltOfLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseSynonymLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseRelatedLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseDerivedLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseAntonymsLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseHypernymsLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseTroponymsLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseFormOfLink
(
    id         SERIAL PRIMARY KEY,
    sense_id    INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseHyponymsLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseMeronymsLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseHolonymsLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseCoordinateTermsLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseProverbsLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseCompoundOfLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE TABLE SenseCategoriesLink
(
    id         SERIAL PRIMARY KEY,
    sense_id   INT,
    category_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (category_id) REFERENCES Categories (id),
    UNIQUE (sense_id, category_id)
);

CREATE INDEX idx_word ON Words (word);
CREATE INDEX idx_lang_code ON Words (lang_code);
CREATE INDEX idx_pos ON Words (pos);
CREATE INDEX idx_translation_word_id ON Translations (word_id);
CREATE INDEX idx_sound_word_id ON Sounds (word_id);
CREATE INDEX idx_template_word_id ON Templates (word_id);
CREATE INDEX idx_forms_word_id ON Forms (word_id);
CREATE INDEX idx_senses_word_id ON Senses (word_id);
CREATE INDEX idx_instances_word_id ON Instance (word_id);
CREATE INDEX idx_forms_ruby_forms_id ON FormsRuby (forms_id);
CREATE INDEX idx_descendants_word_id ON Descendants (word_id);
CREATE INDEX idx_inflection_template_word_id ON InflectionTemplate (word_id);
CREATE INDEX idx_instance_tags_instance_id ON InstanceTags (instance_id);
CREATE INDEX idx_instance_topics_instance_id ON InstanceTopics (instance_id);
CREATE INDEX idx_wikipedia_word_id ON Wikipedia (word_id);
CREATE INDEX idx_wikidata_word_id ON Wikidata (word_id);
CREATE INDEX idx_descendants_tags_descendant_id ON DescendantsTags (descendant_id);
CREATE INDEX idx_descendants_templates_descendant_id ON DescendantsTemplates (descendant_id);
CREATE INDEX idx_senses_tags_sense_id ON SensesTags (sense_id);
CREATE INDEX idx_senses_topics_sense_id ON SensesTopics (sense_id);
CREATE INDEX idx_examples_sense_id ON Examples (sense_id);
CREATE INDEX idx_examples_ruby_example_id ON ExamplesRuby (example_id);
CREATE INDEX idx_head_templates_word_id ON HeadTemplates (word_id);
CREATE INDEX idx_head_templates_template_id ON HeadTemplates (template_id);
CREATE INDEX idx_hyphenation_word_id ON Hyphenations (word_id);
CREATE INDEX idx_categories_word_id_word_id ON WordCategoriesLink (word_id);
CREATE INDEX idx_categories_word_id_category_id ON WordCategoriesLink  (word_id);
CREATE INDEX idx_categories_sense_id_sense_id ON SenseCategoriesLink (sense_id);
CREATE INDEX idx_categories_sense_id_category_id ON SenseCategoriesLink  (sense_id);

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
    id         SERIAL PRIMARY KEY,
    word_id    INT,
    related_id INT,
    FOREIGN KEY (word_id) REFERENCES Words (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (word_id, related_id)
);

CREATE INDEX idx_word_abbreviations_link_word_id ON WordAbbreviationsLink (word_id);
CREATE INDEX idx_word_abbreviations_link_abbreviations_id ON WordAbbreviationsLink (related_id);

CREATE TABLE SenseAbbreviationsLink
(
    id         SERIAL PRIMARY KEY,
    sense_id    INT,
    related_id INT,
    FOREIGN KEY (sense_id) REFERENCES Senses (id),
    FOREIGN KEY (related_id) REFERENCES Related (id),
    UNIQUE (sense_id, related_id)
);

CREATE INDEX idx_sense_abbreviations_link_sense_id ON SenseAbbreviationsLink (sense_id);
CREATE INDEX idx_sense_abbreviations_link_abbreviations_id ON SenseAbbreviationsLink (related_id);