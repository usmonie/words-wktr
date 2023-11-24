// @generated automatically by Diesel CLI.

diesel::table! {
    args (id) {
        id -> Int4,
        etymology_template_id -> Int4,
        arg_key -> Text,
        arg_val -> Text,
    }
}

diesel::table! {
    category_parents (id) {
        id -> Int4,
        category_id -> Int4,
        parent -> Text,
    }
}

diesel::table! {
    categorys (id) {
        id -> Int4,
        sense_id -> Int4,
        name -> Text,
        kind -> Text,
        source -> Text,
        orig -> Nullable<Text>,
        langcode -> Nullable<Text>,
        _dis -> Nullable<Text>,
    }
}

diesel::table! {
    derived_words (id) {
        id -> Int4,
        word_id -> Int4,
        word -> Text,
        _dis1 -> Text,
    }
}

diesel::table! {
    etymology_templates (id) {
        id -> Int4,
        word_id -> Int4,
        name -> Text,
        expansion -> Text,
    }
}

diesel::table! {
    forms (id) {
        id -> Int4,
        word_id -> Int4,
        form -> Text,
    }
}

diesel::table! {
    glosss (id) {
        id -> Int4,
        sense_id -> Int4,
        gloss -> Text,
    }
}

diesel::table! {
    hyphenations (id) {
        id -> Int4,
        word_id -> Int4,
        hyphen -> Text,
    }
}

diesel::table! {
    links (id) {
        id -> Int4,
        sense_id -> Int4,
        link_1 -> Text,
        link_2 -> Text,
    }
}

diesel::table! {
    raw_glosss (id) {
        id -> Int4,
        sense_id -> Int4,
        raw_gloss -> Text,
    }
}

diesel::table! {
    sense_tags (id) {
        id -> Int4,
        sense_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    senses (id) {
        id -> Int4,
        word_id -> Int4,
        id_1 -> Text,
    }
}

diesel::table! {
    sound_tags (id) {
        id -> Int4,
        sound_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    sounds (id) {
        id -> Int4,
        word_id -> Int4,
        ipa -> Nullable<Text>,
        enpr -> Nullable<Text>,
        audio -> Nullable<Text>,
        text -> Nullable<Text>,
        ogg_url -> Nullable<Text>,
        mp3_url -> Nullable<Text>,
        homophone -> Nullable<Text>,
        rhymes -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        form_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    words (id) {
        id -> Int4,
        pos -> Text,
        word -> Text,
        lang_code -> Text,
        lang -> Text,
        etymology_number -> Nullable<Int4>,
        etymology_text -> Nullable<Text>,
    }
}

diesel::joinable!(args -> etymology_templates (etymology_template_id));
diesel::joinable!(category_parents -> categorys (category_id));
diesel::joinable!(categorys -> senses (sense_id));
diesel::joinable!(derived_words -> words (word_id));
diesel::joinable!(etymology_templates -> words (word_id));
diesel::joinable!(forms -> words (word_id));
diesel::joinable!(glosss -> senses (sense_id));
diesel::joinable!(hyphenations -> words (word_id));
diesel::joinable!(links -> senses (sense_id));
diesel::joinable!(raw_glosss -> senses (sense_id));
diesel::joinable!(sense_tags -> senses (sense_id));
diesel::joinable!(senses -> words (word_id));
diesel::joinable!(sound_tags -> sounds (sound_id));
diesel::joinable!(sounds -> words (word_id));
diesel::joinable!(tags -> forms (form_id));

diesel::allow_tables_to_appear_in_same_query!(
    args,
    category_parents,
    categorys,
    derived_words,
    etymology_templates,
    forms,
    glosss,
    hyphenations,
    links,
    raw_glosss,
    sense_tags,
    senses,
    sound_tags,
    sounds,
    tags,
    words,
);
