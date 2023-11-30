// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        langcode -> Nullable<Text>,
        orig -> Nullable<Text>,
        kind -> Nullable<Text>,
        name -> Nullable<Text>,
        source -> Nullable<Text>,
    }
}

diesel::table! {
    descendants (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        depth -> Nullable<Int4>,
        text -> Nullable<Text>,
    }
}

diesel::table! {
    descendantstags (id) {
        id -> Int4,
        descendant_id -> Nullable<Int4>,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    descendantstemplates (id) {
        id -> Int4,
        descendant_id -> Nullable<Int4>,
        template_id -> Nullable<Int4>,
    }
}

diesel::table! {
    examples (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        note -> Nullable<Text>,
        example_ref -> Nullable<Text>,
        roman -> Nullable<Text>,
        english -> Nullable<Text>,
        text -> Nullable<Text>,
        example_type -> Nullable<Text>,
    }
}

diesel::table! {
    examplesruby (id) {
        id -> Int4,
        example_id -> Nullable<Int4>,
        ruby -> Nullable<Text>,
    }
}

diesel::table! {
    forms (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        form -> Nullable<Text>,
        head_nr -> Nullable<Int4>,
        roman -> Nullable<Text>,
        source -> Nullable<Text>,
        ipa -> Nullable<Text>,
    }
}

diesel::table! {
    formsruby (id) {
        id -> Int4,
        forms_id -> Nullable<Int4>,
        ruby -> Nullable<Text>,
    }
}

diesel::table! {
    formstags (id) {
        id -> Int4,
        forms_id -> Nullable<Int4>,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    genericvectorfield (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        field_value -> Nullable<Text>,
    }
}

diesel::table! {
    headtemplates (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        template_id -> Nullable<Int4>,
    }
}

diesel::table! {
    hyphenations (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        hyphenation -> Nullable<Text>,
    }
}

diesel::table! {
    inflectiontemplate (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        args -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    instance (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        sense -> Nullable<Text>,
        source -> Nullable<Text>,
        word -> Nullable<Text>,
    }
}

diesel::table! {
    instancetags (id) {
        id -> Int4,
        instance_id -> Nullable<Int4>,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    instancetopics (id) {
        id -> Int4,
        instance_id -> Nullable<Int4>,
        topic -> Nullable<Text>,
    }
}

diesel::table! {
    related (id) {
        id -> Int4,
        roman -> Nullable<Text>,
        alt -> Nullable<Text>,
        english -> Nullable<Text>,
        sense -> Nullable<Text>,
        word -> Nullable<Text>,
        source -> Nullable<Text>,
        taxonomic -> Nullable<Text>,
        qualifier -> Nullable<Text>,
        extra -> Nullable<Text>,
    }
}

diesel::table! {
    relatedruby (id) {
        id -> Int4,
        related_id -> Nullable<Int4>,
        ruby -> Nullable<Text>,
    }
}

diesel::table! {
    relatedtags (id) {
        id -> Int4,
        related_id -> Nullable<Int4>,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    relatedtopics (id) {
        id -> Int4,
        related_id -> Nullable<Int4>,
        topic -> Nullable<Text>,
    }
}

diesel::table! {
    relatedurls (id) {
        id -> Int4,
        related_id -> Nullable<Int4>,
        url -> Nullable<Text>,
    }
}

diesel::table! {
    senseabbreviationslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sensealtoflink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    senseantonymslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sensecategorieslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        category_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sensecompoundoflink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sensecoordinatetermslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sensederivedlink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    senseformoflink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    senseholonymslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sensehypernymslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sensehyponymslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sensemeronymslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    senseproverbslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    senserelatedlink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    senses (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        sense_id -> Nullable<Text>,
        head_nr -> Nullable<Int4>,
        taxonomic -> Nullable<Text>,
        qualifier -> Nullable<Text>,
        glosses -> Nullable<Text>,
        raw_glosses -> Nullable<Text>,
    }
}

diesel::table! {
    senseslinks (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        links -> Nullable<Text>,
    }
}

diesel::table! {
    sensestags (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    sensestopics (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        topic -> Nullable<Text>,
    }
}

diesel::table! {
    sensesynonymlink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sensetroponymslink (id) {
        id -> Int4,
        sense_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sounds (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
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
    soundtags (id) {
        id -> Int4,
        sound_id -> Nullable<Int4>,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    soundtopics (id) {
        id -> Int4,
        sound_id -> Nullable<Int4>,
        topic -> Nullable<Text>,
    }
}

diesel::table! {
    templates (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        args -> Nullable<Text>,
        name -> Nullable<Text>,
        expansion -> Nullable<Text>,
    }
}

diesel::table! {
    topics (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        topic -> Nullable<Text>,
    }
}

diesel::table! {
    translations (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
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
    translationtags (id) {
        id -> Int4,
        translation_id -> Nullable<Int4>,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    translationtopics (id) {
        id -> Int4,
        translation_id -> Nullable<Int4>,
        topic -> Nullable<Text>,
    }
}

diesel::table! {
    wikidata (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        wikidata_link -> Nullable<Text>,
    }
}

diesel::table! {
    wikipedia (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        wikipedia_link -> Nullable<Text>,
    }
}

diesel::table! {
    wordabbreviationslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordaltoflink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordantonymslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordcategorieslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        category_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordcompoundoflink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordcoordinatetermslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordderivedlink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordformoflink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordholonymslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordhypernymslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordhyponymslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordmeronymslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordproverbslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordrelatedlink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    words (id) {
        id -> Int4,
        word -> Text,
        pos -> Text,
        source -> Nullable<Text>,
        lang_code -> Text,
        lang -> Text,
        original_title -> Nullable<Text>,
        etymology_number -> Nullable<Int4>,
        etymology_text -> Nullable<Text>,
    }
}

diesel::table! {
    wordsynonymlink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::table! {
    wordtroponymslink (id) {
        id -> Int4,
        word_id -> Nullable<Int4>,
        related_id -> Nullable<Int4>,
    }
}

diesel::joinable!(descendants -> words (word_id));
diesel::joinable!(descendantstags -> descendants (descendant_id));
diesel::joinable!(descendantstemplates -> descendants (descendant_id));
diesel::joinable!(descendantstemplates -> templates (template_id));
diesel::joinable!(examples -> senses (sense_id));
diesel::joinable!(examplesruby -> examples (example_id));
diesel::joinable!(forms -> words (word_id));
diesel::joinable!(formsruby -> forms (forms_id));
diesel::joinable!(formstags -> forms (forms_id));
diesel::joinable!(genericvectorfield -> words (word_id));
diesel::joinable!(headtemplates -> templates (template_id));
diesel::joinable!(headtemplates -> words (word_id));
diesel::joinable!(hyphenations -> words (word_id));
diesel::joinable!(inflectiontemplate -> words (word_id));
diesel::joinable!(instance -> words (word_id));
diesel::joinable!(instancetags -> instance (instance_id));
diesel::joinable!(instancetopics -> instance (instance_id));
diesel::joinable!(relatedruby -> related (related_id));
diesel::joinable!(relatedtags -> related (related_id));
diesel::joinable!(relatedtopics -> related (related_id));
diesel::joinable!(relatedurls -> related (related_id));
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
diesel::joinable!(senseslinks -> senses (sense_id));
diesel::joinable!(sensestags -> senses (sense_id));
diesel::joinable!(sensestopics -> senses (sense_id));
diesel::joinable!(sensesynonymlink -> related (related_id));
diesel::joinable!(sensesynonymlink -> senses (sense_id));
diesel::joinable!(sensetroponymslink -> related (related_id));
diesel::joinable!(sensetroponymslink -> senses (sense_id));
diesel::joinable!(sounds -> words (word_id));
diesel::joinable!(soundtags -> sounds (sound_id));
diesel::joinable!(soundtopics -> sounds (sound_id));
diesel::joinable!(templates -> words (word_id));
diesel::joinable!(topics -> words (word_id));
diesel::joinable!(translations -> words (word_id));
diesel::joinable!(translationtags -> translations (translation_id));
diesel::joinable!(translationtopics -> translations (translation_id));
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
    descendantstags,
    descendantstemplates,
    examples,
    examplesruby,
    forms,
    formsruby,
    formstags,
    genericvectorfield,
    headtemplates,
    hyphenations,
    inflectiontemplate,
    instance,
    instancetags,
    instancetopics,
    related,
    relatedruby,
    relatedtags,
    relatedtopics,
    relatedurls,
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
    senseslinks,
    sensestags,
    sensestopics,
    sensesynonymlink,
    sensetroponymslink,
    sounds,
    soundtags,
    soundtopics,
    templates,
    topics,
    translations,
    translationtags,
    translationtopics,
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
