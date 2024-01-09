// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        langcode -> Nullable<Text>,
        orig -> Nullable<Text>,
        kind -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    descendants (id) {
        id -> Int4,
        word_id -> Int4,
        depth -> Nullable<Int4>,
        text -> Nullable<Text>,
    }
}

diesel::table! {
    descendants_tags (descendant_id, tag) {
        id -> Int4,
        descendant_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    descendants_templates (descendant_id, template_id) {
        id -> Int4,
        descendant_id -> Int4,
        template_id -> Int4,
    }
}

diesel::table! {
    example_ruby (example_id, ruby) {
        id -> Int4,
        example_id -> Int4,
        ruby -> Text,
    }
}

diesel::table! {
    examples (id) {
        id -> Int4,
        sense_id -> Int4,
        note -> Nullable<Text>,
        example_ref -> Nullable<Text>,
        roman -> Nullable<Text>,
        english -> Nullable<Text>,
        text -> Nullable<Text>,
        example_type -> Nullable<Text>,
    }
}

diesel::table! {
    forms (id) {
        id -> Int4,
        word_id -> Int4,
        form -> Nullable<Text>,
        head_nr -> Nullable<Int4>,
        roman -> Nullable<Text>,
        ipa -> Nullable<Text>,
    }
}

diesel::table! {
    forms_ruby (forms_id, ruby) {
        id -> Int4,
        forms_id -> Int4,
        ruby -> Text,
    }
}

diesel::table! {
    forms_tags (forms_id, tag) {
        id -> Int4,
        forms_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    hyphenations (id) {
        id -> Int4,
        word_id -> Int4,
        hyphenation -> Nullable<Text>,
    }
}

diesel::table! {
    inflectiontemplate (id) {
        id -> Int4,
        word_id -> Int4,
        args -> Nullable<Text>,
        name -> Text,
    }
}

diesel::table! {
    instance (id) {
        id -> Int4,
        word_id -> Int4,
        sense -> Nullable<Text>,
        word -> Nullable<Text>,
    }
}

diesel::table! {
    instance_tags (instance_id, tag) {
        id -> Int4,
        instance_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    instance_topics (instance_id, topic) {
        id -> Int4,
        instance_id -> Int4,
        topic -> Text,
    }
}

diesel::table! {
    links (link) {
        link -> Text,
    }
}

diesel::table! {
    related (id) {
        id -> Int4,
        roman -> Nullable<Text>,
        alt -> Nullable<Text>,
        english -> Nullable<Text>,
        sense -> Nullable<Text>,
        word -> Text,
        taxonomic -> Nullable<Text>,
        qualifier -> Nullable<Text>,
        extra -> Nullable<Text>,
    }
}

diesel::table! {
    related_ruby (related_id, ruby) {
        id -> Int4,
        related_id -> Int4,
        ruby -> Text,
    }
}

diesel::table! {
    related_tags (related_id, tag) {
        id -> Int4,
        related_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    related_topics (related_id, topic) {
        id -> Int4,
        related_id -> Int4,
        topic -> Text,
    }
}

diesel::table! {
    related_urls (related_id, url) {
        id -> Int4,
        related_id -> Int4,
        url -> Text,
    }
}

diesel::table! {
    rubies (ruby) {
        ruby -> Text,
    }
}

diesel::table! {
    sense_links (sense_id, link) {
        id -> Int4,
        sense_id -> Int4,
        link -> Text,
    }
}

diesel::table! {
    sense_tags (sense_id, tag) {
        id -> Int4,
        sense_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    sense_topics (sense_id, topic) {
        id -> Int4,
        sense_id -> Int4,
        topic -> Text,
    }
}

diesel::table! {
    senseabbreviationslink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    sensealtoflink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    senseantonymslink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    sensecategorieslink (sense_id, category_id) {
        id -> Int4,
        sense_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    sensecompoundoflink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    sensecoordinatetermslink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    sensederivedlink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    senseformoflink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    senseholonymslink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    sensehypernymslink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    sensehyponymslink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    sensemeronymslink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    senseproverbslink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    senserelatedlink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    senses (id) {
        id -> Int4,
        word_id -> Int4,
        sense_id -> Nullable<Text>,
        head_nr -> Nullable<Int4>,
        taxonomic -> Nullable<Text>,
        qualifier -> Nullable<Text>,
        glosses -> Nullable<Text>,
        raw_glosses -> Nullable<Text>,
    }
}

diesel::table! {
    sensesynonymlink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    sensetroponymslink (sense_id, related_id) {
        id -> Int4,
        sense_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    sound_tags (sound_id, tag) {
        id -> Int4,
        sound_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    sound_topics (sound_id, topic) {
        id -> Int4,
        sound_id -> Int4,
        topic -> Text,
    }
}

diesel::table! {
    sounds (id) {
        id -> Int4,
        word_id -> Int4,
        mp3_url -> Nullable<Text>,
        note -> Nullable<Text>,
        rhymes -> Nullable<Text>,
        other -> Nullable<Text>,
        enpr -> Nullable<Text>,
        audio_ipa -> Nullable<Text>,
        ogg_url -> Nullable<Text>,
        form -> Nullable<Text>,
        ipa -> Nullable<Text>,
        audio -> Nullable<Text>,
        text -> Nullable<Text>,
        homophone -> Nullable<Text>,
        zh_pron -> Nullable<Text>,
    }
}

diesel::table! {
    tags (tag) {
        tag -> Text,
    }
}

diesel::table! {
    templates (id) {
        id -> Int4,
        word_id -> Int4,
        args -> Nullable<Text>,
        name -> Nullable<Text>,
        expansion -> Nullable<Text>,
    }
}

diesel::table! {
    topics (topic) {
        topic -> Text,
    }
}

diesel::table! {
    translation_tags (translation_id, tag) {
        id -> Int4,
        translation_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    translation_topics (translation_id, topic) {
        id -> Int4,
        translation_id -> Int4,
        topic -> Text,
    }
}

diesel::table! {
    translations (id) {
        id -> Int4,
        note -> Nullable<Text>,
        code -> Nullable<Text>,
        roman -> Nullable<Text>,
        alt -> Nullable<Text>,
        english -> Nullable<Text>,
        sense -> Nullable<Text>,
        lang -> Nullable<Text>,
        word -> Nullable<Text>,
        taxonomic -> Nullable<Text>,
    }
}

diesel::table! {
    translations_words_link (translation_id, word_id) {
        id -> Int4,
        translation_id -> Int4,
        word_id -> Int4,
    }
}

diesel::table! {
    urls (url) {
        url -> Text,
    }
}

diesel::table! {
    wikidata (id) {
        id -> Int4,
        word_id -> Int4,
        wikidata_link -> Text,
    }
}

diesel::table! {
    wikipedia (id) {
        id -> Int4,
        word_id -> Int4,
        wikipedia_link -> Text,
    }
}

diesel::table! {
    wordabbreviationslink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordaltoflink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordantonymslink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordcategorieslink (word_id, category_id) {
        id -> Int4,
        word_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    wordcompoundoflink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordcoordinatetermslink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordderivedlink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordformoflink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordholonymslink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordhypernymslink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordhyponymslink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordmeronymslink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordproverbslink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordrelatedlink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    words (id) {
        id -> Int4,
        word -> Text,
        pos -> Text,
        lang_code -> Text,
        lang -> Text,
        etymology_number -> Nullable<Int4>,
        etymology_text -> Nullable<Text>,
    }
}

diesel::table! {
    wordsynonymlink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::table! {
    wordtroponymslink (word_id, related_id) {
        id -> Int4,
        word_id -> Int4,
        related_id -> Int4,
    }
}

diesel::joinable!(descendants -> words (word_id));
diesel::joinable!(descendants_tags -> descendants (descendant_id));
diesel::joinable!(descendants_tags -> tags (tag));
diesel::joinable!(descendants_templates -> descendants (descendant_id));
diesel::joinable!(descendants_templates -> templates (template_id));
diesel::joinable!(example_ruby -> examples (example_id));
diesel::joinable!(example_ruby -> rubies (ruby));
diesel::joinable!(examples -> senses (sense_id));
diesel::joinable!(forms -> words (word_id));
diesel::joinable!(forms_ruby -> forms (forms_id));
diesel::joinable!(forms_ruby -> rubies (ruby));
diesel::joinable!(forms_tags -> forms (forms_id));
diesel::joinable!(forms_tags -> tags (tag));
diesel::joinable!(hyphenations -> words (word_id));
diesel::joinable!(inflectiontemplate -> words (word_id));
diesel::joinable!(instance -> words (word_id));
diesel::joinable!(instance_tags -> instance (instance_id));
diesel::joinable!(instance_tags -> tags (tag));
diesel::joinable!(instance_topics -> instance (instance_id));
diesel::joinable!(instance_topics -> topics (topic));
diesel::joinable!(related_ruby -> related (related_id));
diesel::joinable!(related_ruby -> rubies (ruby));
diesel::joinable!(related_tags -> related (related_id));
diesel::joinable!(related_tags -> tags (tag));
diesel::joinable!(related_topics -> related (related_id));
diesel::joinable!(related_topics -> topics (topic));
diesel::joinable!(related_urls -> related (related_id));
diesel::joinable!(related_urls -> urls (url));
diesel::joinable!(sense_links -> links (link));
diesel::joinable!(sense_links -> senses (sense_id));
diesel::joinable!(sense_tags -> senses (sense_id));
diesel::joinable!(sense_tags -> tags (tag));
diesel::joinable!(sense_topics -> senses (sense_id));
diesel::joinable!(sense_topics -> topics (topic));
diesel::joinable!(senseabbreviationslink -> related (related_id));
diesel::joinable!(senseabbreviationslink -> senses (sense_id));
diesel::joinable!(sensealtoflink -> related (related_id));
diesel::joinable!(sensealtoflink -> senses (sense_id));
diesel::joinable!(senseantonymslink -> related (related_id));
diesel::joinable!(senseantonymslink -> senses (sense_id));
diesel::joinable!(sensecategorieslink -> categories (category_id));
diesel::joinable!(sensecategorieslink -> senses (sense_id));
diesel::joinable!(sensecompoundoflink -> related (related_id));
diesel::joinable!(sensecompoundoflink -> senses (sense_id));
diesel::joinable!(sensecoordinatetermslink -> related (related_id));
diesel::joinable!(sensecoordinatetermslink -> senses (sense_id));
diesel::joinable!(sensederivedlink -> related (related_id));
diesel::joinable!(sensederivedlink -> senses (sense_id));
diesel::joinable!(senseformoflink -> related (related_id));
diesel::joinable!(senseformoflink -> senses (sense_id));
diesel::joinable!(senseholonymslink -> related (related_id));
diesel::joinable!(senseholonymslink -> senses (sense_id));
diesel::joinable!(sensehypernymslink -> related (related_id));
diesel::joinable!(sensehypernymslink -> senses (sense_id));
diesel::joinable!(sensehyponymslink -> related (related_id));
diesel::joinable!(sensehyponymslink -> senses (sense_id));
diesel::joinable!(sensemeronymslink -> related (related_id));
diesel::joinable!(sensemeronymslink -> senses (sense_id));
diesel::joinable!(senseproverbslink -> related (related_id));
diesel::joinable!(senseproverbslink -> senses (sense_id));
diesel::joinable!(senserelatedlink -> related (related_id));
diesel::joinable!(senserelatedlink -> senses (sense_id));
diesel::joinable!(senses -> words (word_id));
diesel::joinable!(sensesynonymlink -> related (related_id));
diesel::joinable!(sensesynonymlink -> senses (sense_id));
diesel::joinable!(sensetroponymslink -> related (related_id));
diesel::joinable!(sensetroponymslink -> senses (sense_id));
diesel::joinable!(sound_tags -> sounds (sound_id));
diesel::joinable!(sound_tags -> tags (tag));
diesel::joinable!(sound_topics -> sounds (sound_id));
diesel::joinable!(sound_topics -> topics (topic));
diesel::joinable!(sounds -> words (word_id));
diesel::joinable!(templates -> words (word_id));
diesel::joinable!(translation_tags -> tags (tag));
diesel::joinable!(translation_tags -> translations (translation_id));
diesel::joinable!(translation_topics -> topics (topic));
diesel::joinable!(translation_topics -> translations (translation_id));
diesel::joinable!(translations_words_link -> translations (translation_id));
diesel::joinable!(translations_words_link -> words (word_id));
diesel::joinable!(wikidata -> words (word_id));
diesel::joinable!(wikipedia -> words (word_id));
diesel::joinable!(wordabbreviationslink -> related (related_id));
diesel::joinable!(wordabbreviationslink -> words (word_id));
diesel::joinable!(wordaltoflink -> related (related_id));
diesel::joinable!(wordaltoflink -> words (word_id));
diesel::joinable!(wordantonymslink -> related (related_id));
diesel::joinable!(wordantonymslink -> words (word_id));
diesel::joinable!(wordcategorieslink -> categories (category_id));
diesel::joinable!(wordcategorieslink -> words (word_id));
diesel::joinable!(wordcompoundoflink -> related (related_id));
diesel::joinable!(wordcompoundoflink -> words (word_id));
diesel::joinable!(wordcoordinatetermslink -> related (related_id));
diesel::joinable!(wordcoordinatetermslink -> words (word_id));
diesel::joinable!(wordderivedlink -> related (related_id));
diesel::joinable!(wordderivedlink -> words (word_id));
diesel::joinable!(wordformoflink -> related (related_id));
diesel::joinable!(wordformoflink -> words (word_id));
diesel::joinable!(wordholonymslink -> related (related_id));
diesel::joinable!(wordholonymslink -> words (word_id));
diesel::joinable!(wordhypernymslink -> related (related_id));
diesel::joinable!(wordhypernymslink -> words (word_id));
diesel::joinable!(wordhyponymslink -> related (related_id));
diesel::joinable!(wordhyponymslink -> words (word_id));
diesel::joinable!(wordmeronymslink -> related (related_id));
diesel::joinable!(wordmeronymslink -> words (word_id));
diesel::joinable!(wordproverbslink -> related (related_id));
diesel::joinable!(wordproverbslink -> words (word_id));
diesel::joinable!(wordrelatedlink -> related (related_id));
diesel::joinable!(wordrelatedlink -> words (word_id));
diesel::joinable!(wordsynonymlink -> related (related_id));
diesel::joinable!(wordsynonymlink -> words (word_id));
diesel::joinable!(wordtroponymslink -> related (related_id));
diesel::joinable!(wordtroponymslink -> words (word_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    descendants,
    descendants_tags,
    descendants_templates,
    example_ruby,
    examples,
    forms,
    forms_ruby,
    forms_tags,
    hyphenations,
    inflectiontemplate,
    instance,
    instance_tags,
    instance_topics,
    links,
    related,
    related_ruby,
    related_tags,
    related_topics,
    related_urls,
    rubies,
    sense_links,
    sense_tags,
    sense_topics,
    senseabbreviationslink,
    sensealtoflink,
    senseantonymslink,
    sensecategorieslink,
    sensecompoundoflink,
    sensecoordinatetermslink,
    sensederivedlink,
    senseformoflink,
    senseholonymslink,
    sensehypernymslink,
    sensehyponymslink,
    sensemeronymslink,
    senseproverbslink,
    senserelatedlink,
    senses,
    sensesynonymlink,
    sensetroponymslink,
    sound_tags,
    sound_topics,
    sounds,
    tags,
    templates,
    topics,
    translation_tags,
    translation_topics,
    translations,
    translations_words_link,
    urls,
    wikidata,
    wikipedia,
    wordabbreviationslink,
    wordaltoflink,
    wordantonymslink,
    wordcategorieslink,
    wordcompoundoflink,
    wordcoordinatetermslink,
    wordderivedlink,
    wordformoflink,
    wordholonymslink,
    wordhypernymslink,
    wordhyponymslink,
    wordmeronymslink,
    wordproverbslink,
    wordrelatedlink,
    words,
    wordsynonymlink,
    wordtroponymslink,
);
