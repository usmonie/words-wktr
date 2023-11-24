use diesel::{AsChangeset, Associations, Identifiable, Queryable};
use serde::Serialize;

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(EtymologyTemplate, foreign_key = "etymology_template_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::args)]
pub struct Arg {
    pub id: i32,
    pub etymology_template_id: i32,
    pub arg_key: String,
    pub arg_val: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Sense, foreign_key = "sense_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::categorys)]
pub struct Category {
    pub id: i32,
    pub sense_id: i32,
    pub name: String,
    pub kind: String,
    pub source: String,
    pub orig: Option<String>,
    pub langcode: Option<String>,
    pub _dis: Option<String>,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Category, foreign_key = "category_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::category_parents)]
pub struct CategoryParent {
    pub id: i32,
    pub category_id: i32,
    pub parent: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Word, foreign_key = "word_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::derived_words)]
pub struct DerivedWord {
    pub id: i32,
    pub word_id: i32,
    pub word: String,
    pub _dis1: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Word, foreign_key = "word_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::etymology_templates)]
pub struct EtymologyTemplate {
    pub id: i32,
    pub word_id: i32,
    pub name: String,
    pub expansion: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Word, foreign_key = "word_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::forms)]
pub struct Form {
    pub id: i32,
    pub word_id: i32,
    pub form: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Sense, foreign_key = "sense_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::glosss)]
pub struct Gloss {
    pub id: i32,
    pub sense_id: i32,
    pub gloss: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Word, foreign_key = "word_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::hyphenations)]
pub struct Hyphenation {
    pub id: i32,
    pub word_id: i32,
    pub hyphen: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Sense, foreign_key = "sense_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::links)]
pub struct Link {
    pub id: i32,
    pub sense_id: i32,
    pub link_1: String,
    pub link_2: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Sense, foreign_key = "sense_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::raw_glosss)]
pub struct RawGloss {
    pub id: i32,
    pub sense_id: i32,
    pub raw_gloss: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Sense)]
#[primary_key(id)]
#[diesel(table_name = crate::schema::sense_tags)]
pub struct SenseTag {
    pub id: i32,
    pub sense_id: i32,
    pub tag: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize, Clone)]
#[belongs_to(Word, foreign_key = "word_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::senses)]
pub struct Sense {
    pub id: i32,
    pub word_id: i32,
    pub id_1: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Sound, foreign_key = "sound_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::sound_tags)]
pub struct SoundTag {
    pub id: i32,
    pub sound_id: i32,
    pub tag: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Word, foreign_key = "word_id")]
#[primary_key(id)]
#[diesel(table_name = crate::schema::sounds)]
pub struct Sound {
    pub id: i32,
    pub word_id: i32,
    pub ipa: Option<String>,
    pub enpr: Option<String>,
    pub audio: Option<String>,
    pub text: Option<String>,
    pub ogg_url: Option<String>,
    pub mp3_url: Option<String>,
    pub homophone: Option<String>,
    pub rhymes: Option<String>,
}

#[derive(Debug, Queryable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Form)]
#[primary_key(id)]
#[diesel(table_name = crate::schema::tags)]
pub struct Tag {
    pub id: i32,
    pub form_id: i32,
    pub tag: String,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset, Serialize, Clone)]
#[primary_key(id)]
#[diesel(table_name = crate::schema::words)]
pub struct Word {
    pub id: i32,
    pub pos: String,
    pub word: String,
    pub lang_code: String,
    pub lang: String,
    pub etymology_number: Option<i32>,
    pub etymology_text: Option<String>,
}

use diesel::Insertable;
use serde::Deserialize;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::args)]
pub struct NewArg {
    pub etymology_template_id: i32,
    pub arg_key: Option<String>,
    pub arg_val: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::categorys)]
pub struct NewCategory {
    pub sense_id: i32,
    pub name: String,
    pub kind: String,
    pub source: String,
    pub orig: Option<String>,
    pub langcode: Option<String>,
    pub _dis: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::category_parents)]
pub struct NewCategoryParent {
    pub category_id: i32,
    pub parent: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::derived_words)]
pub struct NewDerivedWord {
    pub word_id: i32,
    pub word: String,
    pub _dis1: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::etymology_templates)]
pub struct NewEtymologyTemplate {
    pub word_id: i32,
    pub name: String,
    pub expansion: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::forms)]
pub struct NewForm {
    pub word_id: i32,
    pub form: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::glosss)]
pub struct NewGloss {
    pub sense_id: i32,
    pub gloss: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::hyphenations)]
pub struct NewHyphenation {
    pub word_id: i32,
    pub hyphen: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::links)]
pub struct NewLink {
    pub sense_id: i32,
    pub link_1: Option<String>,
    pub link_2: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::raw_glosss)]
pub struct NewRawGloss {
    pub sense_id: i32,
    pub raw_gloss: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::sense_tags)]
pub struct NewSenseTag {
    pub sense_id: i32,
    pub tag: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::senses)]
pub struct NewSense {
    pub word_id: i32,
    pub id_1: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::sound_tags)]
pub struct NewSoundTag {
    pub sound_id: i32,
    pub tag: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::sounds)]
pub struct NewSound {
    pub word_id: i32,
    pub ipa: Option<String>,
    pub enpr: Option<String>,
    pub audio: Option<String>,
    pub text: Option<String>,
    pub ogg_url: Option<String>,
    pub mp3_url: Option<String>,
    pub homophone: Option<String>,
    pub rhymes: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::tags)]
pub struct NewTag {
    pub form_id: i32,
    pub tag: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::words)]
pub struct NewWord {
    pub pos: String,
    pub word: String,
    pub lang_code: String,
    pub lang: String,
    pub etymology_number: Option<i32>,
    pub etymology_text: Option<String>,
}
