// Import Diesel traits
use diesel::{Identifiable, Insertable, Selectable};
use diesel::Queryable;
use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "words"]
pub struct NewWord {
    pub word: String,
    pub pos: String,
    pub lang_code: String,
    pub lang: String,
    pub etymology_number: Option<i32>,
    pub etymology_text: Option<String>,
}

#[derive(Insertable)]
#[table_name = "related"]
pub struct NewRelated {
    pub roman: Option<String>,
    pub alt: Option<String>,
    pub english: Option<String>,
    pub sense: Option<String>,
    pub word: Option<String>,
    pub taxonomic: Option<String>,
    pub qualifier: Option<String>,
    pub extra: Option<String>,
}

#[derive(Insertable)]
#[table_name = "templates"]
pub struct NewTemplate {
    pub word_id: i32,
    pub args: String,
    pub name: Option<String>,
    pub expansion: Option<String>,
}

#[derive(Insertable)]
#[table_name = "translations"]
pub struct NewTranslation {
    pub note: Option<String>,
    pub code: Option<String>,
    pub roman: Option<String>,
    pub alt: Option<String>,
    pub english: Option<String>,
    pub sense: Option<String>,
    pub lang: Option<String>,
    pub word: Option<String>,
    pub taxonomic: Option<String>,
}


#[derive(Insertable)]
#[diesel(primary_key(translation_id, word_id))]
#[diesel(table_name=crate::schema::translations_words_link)]
pub struct NewTranslationsWordsLink {
    pub translation_id: i32,
    pub word_id: i32,
}

#[derive(Insertable)]
#[table_name = "sounds"]
pub struct NewSound {
    pub word_id: i32,
    pub mp3_url: Option<String>,
    pub note: Option<String>,
    pub rhymes: Option<String>,
    pub other: Option<String>,
    pub enpr: Option<String>,
    pub audio_ipa: Option<String>,
    pub ogg_url: Option<String>,
    pub form: Option<String>,
    pub ipa: Option<String>,
    pub audio: Option<String>,
    pub text: Option<String>,
    pub homophone: Option<String>,
    pub zh_pron: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "categories"]
pub struct NewCategorie {
    pub langcode: Option<String>,
    pub orig: Option<String>,
    pub kind: Option<String>,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "forms"]
pub struct NewForm {
    pub word_id: i32,
    pub form: Option<String>,
    pub head_nr: Option<i32>,
    pub roman: Option<String>,
    pub ipa: Option<String>,
}

#[derive(Insertable)]
#[table_name = "wikipedia"]
pub struct NewWikipedia {
    pub word_id: i32,
    pub wikipedia_link: String,
}

#[derive(Insertable)]
#[table_name = "wikidata"]
pub struct NewWikidata {
    pub word_id: i32,
    pub wikidata_link: String,
}

#[derive(Insertable)]
#[table_name = "hyphenations"]
pub struct NewHyphenation {
    pub word_id: i32,
    pub hyphenation: String,
}

#[derive(Insertable)]
#[table_name = "senses"]
pub struct NewSense {
    pub word_id: i32,
    pub sense_id: Option<String>,
    pub head_nr: Option<i32>,
    pub taxonomic: Option<String>,
    pub qualifier: Option<String>,
    pub glosses: Option<String>,
    pub raw_glosses: Option<String>,
}

#[derive(Insertable)]
#[table_name = "examples"]
pub struct NewExample {
    pub sense_id: i32,
    pub note: Option<String>,
    pub example_ref: Option<String>,
    pub roman: Option<String>,
    pub english: Option<String>,
    pub text: Option<String>,
    pub example_type: Option<String>,
}

#[derive(Insertable)]
#[table_name = "instance"]
pub struct NewInstance {
    pub word_id: i32,
    pub sense: Option<String>,
    pub word: Option<String>,
}

#[derive(Insertable)]
#[table_name = "descendants"]
pub struct NewDescendants {
    pub word_id: i32,
    pub depth: Option<i32>,
    pub text: Option<String>,
}

// #[derive(Insertable)]
// #[table_name = "descendantstemplates"]
// pub struct NewDescendantsTemplates {
//     pub descendant_id: i32,
//     pub template_id: i32,
// }

// #[derive(Queryable, Serialize, Selectable, Associations, Debug, PartialEq)]
// #[diesel(belongs_to(Related))]
// #[table_name = "relatedlinks"]
// pub struct RelatedLinks {
//     pub id: i32,
//     pub related_id: i32,
//     pub links: String,
// }
//
//
#[derive(Queryable, Serialize, Selectable, Insertable, Debug, PartialEq)]
#[table_name = "rubies"]
pub struct Ruby {
    pub ruby: String,
}
//
//#[derive(Queryable, Serialize, Selectable, Insertable, Debug, PartialEq)]
//pub struct Link {
//    pub link: String,
//}
//
//#[derive(Queryable, Insertable, Serialize, Selectable, Debug, PartialEq)]
//#[table_name = "tags"]
//pub struct Tag {
//    pub tag: String,
//}
//
//#[derive(Queryable, Serialize, Selectable, Insertable, Debug, PartialEq)]
//#[table_name = "topics"]
//pub struct Topic {
//    pub topic: String,
//}
//
//#[derive(Queryable, Serialize, Selectable, Insertable, Debug, PartialEq)]
//#[table_name = "urls"]
//pub struct Url {
//    pub url: String,
//}
//
