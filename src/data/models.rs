// Import Diesel traits
use diesel::{Insertable, Selectable};
use diesel::Queryable;
use crate::schema::*;

#[derive(Queryable)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub pos: String,
    pub source: Option<String>,
    pub lang_code: String,
    pub lang: String,
    pub original_title: Option<String>,
    pub etymology_number: Option<i64>,
    pub etymology_text: Option<String>,
}

#[derive(Insertable)]
#[table_name = "words"]
pub struct NewWord {
    pub word: String,
    pub pos: String,
    pub source: Option<String>,
    pub lang_code: String,
    pub lang: String,
    pub original_title: Option<String>,
    pub etymology_number: Option<i32>,
    pub etymology_text: Option<String>,
}

#[derive(Queryable)]
// #[table_name = "related"]
pub struct Related {
    pub id: i32,
    pub roman: Option<String>,
    pub alt: Option<String>,
    pub english: Option<String>,
    pub sense: Option<String>,
    pub word: Option<String>,
    pub source: Option<String>,
    pub taxonomic: Option<String>,
    pub qualifier: Option<String>,
    pub extra: Option<String>,
}

#[derive(Insertable)]
#[table_name = "related"]
pub struct NewRelated {
    pub roman: Option<String>,
    pub alt: Option<String>,
    pub english: Option<String>,
    pub sense: Option<String>,
    pub word: Option<String>,
    pub source: Option<String>,
    pub taxonomic: Option<String>,
    pub qualifier: Option<String>,
    pub extra: Option<String>,
}

#[derive(Queryable)]
pub struct SenseLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Queryable)]
pub struct SenseTag {
    pub sense_id: i32,
    pub _id: i32,
}

#[derive(Queryable)]
pub struct WordLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordcompoundoflink"]
pub struct NewWordCompoundOfLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensecompoundoflink"]
pub struct NewSensesSynonymLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordsynonymlink"]
pub struct NewWordSynonymLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensesynonymlink"]
pub struct NewSenseSynonymLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordtroponymslink"]
pub struct NewWordTroponymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensetroponymslink"]
pub struct NewSenseTroponymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordantonymslink"]
pub struct NewWordAntonymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "senseantonymslink"]
pub struct NewSenseAntonymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordcoordinatetermslink"]
pub struct NewWordCoordinateTermsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensecoordinatetermslink"]
pub struct NewSenseCoordinateTermsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordmeronymslink"]
pub struct NewWordMeronymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensemeronymslink"]
pub struct NewSenseMeronymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordrelatedlink"]
pub struct NewWordRelatedLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "senserelatedlink"]
pub struct NewSenseRelatedLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordderivedlink"]
pub struct NewWordDerivedLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensederivedlink"]
pub struct NewSenseDerivedLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordholonymslink"]
pub struct NewWordHolonymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "senseholonymslink"]
pub struct NewSenseHolonymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordhypernymslink"]
pub struct NewWordHypernymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensehypernymslink"]
pub struct NewSenseHypernymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordhyponymslink"]
pub struct NewWordHyponymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensehyponymslink"]
pub struct NewSenseHyponymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordcategorieslink"]
pub struct NewWordCategoriesLink {
    pub word_id: i32,
    pub category_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensecategorieslink"]
pub struct NewSenseCategoriesLink {
    pub sense_id: i32,
    pub category_id: i32,
}


#[derive(Insertable)]
#[table_name = "wordabbreviationslink"]
pub struct NewWordAbbreviationsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "senseabbreviationslink"]
pub struct NewSenseAbbreviationsLink  {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordformoflink"]
pub struct NewWordFormOfLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "senseformoflink"]
pub struct NewSenseFormOfLink  {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordproverbslink"]
pub struct NewWordProverbLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "senseproverbslink"]
pub struct NewSenseProverbLink  {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "wordaltoflink"]
pub struct NewWordAltOfLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable)]
#[table_name = "sensealtoflink"]
pub struct NewSenseAltOfLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Queryable)]
pub struct Template {
    pub id: i32,
    pub word_id: i32,
    pub args: String, // Assuming args is stored as serialized text
    pub name: Option<String>,
    pub expansion: Option<String>,
}

#[derive(Insertable)]
#[table_name = "templates"]
pub struct NewTemplate {
    pub word_id: i32,
    pub args: String, // Assuming args is provided as serialized text
    pub name: Option<String>,
    pub expansion: Option<String>,
}

#[derive(Queryable)]
pub struct Translation {
    pub id: i32,
    pub word_id: i32,
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
#[table_name = "translations"]
pub struct NewTranslation {
    pub word_id: i32,
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

#[derive(Queryable)]
pub struct Sound {
    pub id: i32,
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

#[derive(Queryable)]
pub struct SoundTags {
    pub id: i32,
    pub sound_id: i32,
    pub tag: String,
}

#[derive(Insertable)]
#[table_name = "soundtags"]
pub struct NewSoundTags {
    pub sound_id: i32,
    pub tag: String,
}

#[derive(Queryable,)]
pub struct Categories {
    pub id: i32,
    pub langcode: Option<String>,
    pub orig: Option<String>,
    pub kind: Option<String>,
    pub name: Option<String>,
    pub source: Option<String>,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategories {
    pub langcode: Option<String>,
    pub orig: Option<String>,
    pub kind: Option<String>,
    pub name: Option<String>,
    pub source: Option<String>,
}

#[derive(Queryable)]
pub struct Forms {
    pub id: i32,
    pub word_id: i32,
    pub form: Option<String>,
    pub head_nr: Option<i32>,
    pub roman: Option<String>,
    pub source: Option<String>,
    pub ipa: Option<String>,
}

#[derive(Insertable)]
#[table_name = "forms"]
pub struct NewForm {
    pub word_id: i32,
    pub form: Option<String>,
    pub head_nr: Option<i32>,
    pub roman: Option<String>,
    pub source: Option<String>,
    pub ipa: Option<String>,
}

#[derive(Queryable)]
pub struct Wikipedia {
    pub id: i32,
    pub word_id: i32,
    pub wikipedia_link: String,
}

#[derive(Insertable)]
#[table_name = "wikipedia"]
pub struct NewWikipedia {
    pub word_id: i32,
    pub wikipedia_link: String,
}

#[derive(Queryable)]
pub struct Wikidata {
    pub id: i32,
    pub word_id: i32,
    pub wikidata_link: String,
}

#[derive(Insertable)]
#[table_name = "wikidata"]
pub struct NewWikidata {
    pub word_id: i32,
    pub wikidata_link: String,
}

#[derive(Queryable)]
pub struct Hyphenation {
    pub id: i32,
    pub word_id: i32,
    pub hyphenation: String,
}

#[derive(Insertable)]
#[table_name = "hyphenations"]
pub struct NewHyphenation {
    pub word_id: i32,
    pub hyphenation: String,
}

#[derive(Queryable)]
pub struct Sense {
    pub id: i32,
    pub word_id: i32,
    pub sense_id: Option<String>,
    pub head_nr: Option<i32>,
    pub taxonomic: Option<String>,
    pub qualifier: Option<String>,
    pub glosses: Option<String>,
    pub raw_glosses: Option<String>
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
    pub raw_glosses: Option<String>
}

#[derive(Queryable)]
pub struct Example {
    pub id: i32,
    pub sense_id: i32,
    pub note: Option<String>,
    pub example_ref: Option<String>,
    pub roman: Option<String>,
    pub english: Option<String>,
    pub text: Option<String>,
    pub example_type: Option<String>,
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

#[derive(Queryable)]
pub struct FormsRuby {
    pub id: i32,
    pub forms_id: i32,
    pub ruby: String,
}

#[derive(Insertable)]
#[table_name = "formsruby"]
pub struct NewFormsRuby {
    pub forms_id: i32,
    pub ruby: String,
}

#[derive(Queryable)]
pub struct Instance {
    pub id: i32,
    pub word_id: i32,
    pub sense: Option<String>,
    pub source: Option<String>,
    pub word: Option<String>,
}

#[derive(Insertable)]
#[table_name = "instance"]
pub struct NewInstance {
    pub word_id: i32,
    pub sense: Option<String>,
    pub source: Option<String>,
    pub word: Option<String>,
}

#[derive(Queryable)]
pub struct InstanceTags {
    pub id: i32,
    pub instance_id: i32,
    pub tag: String,
}

#[derive(Insertable)]
#[table_name = "instancetags"]
pub struct NewInstanceTags {
    pub instance_id: i32,
    pub tag: String,
}

#[derive(Queryable)]
pub struct FormsTags {
    pub id: i32,
    pub forms_id: i32,
    pub tag: String,
}

#[derive(Insertable)]
#[table_name = "formstags"]
pub struct NewFormsTags {
    pub forms_id: i32,
    pub tag: String,
}

#[derive(Queryable)]
pub struct InstanceTopics {
    pub id: i32,
    pub instance_id: i32,
    pub topic: String,
}

#[derive(Insertable)]
#[table_name = "instancetopics"]
pub struct NewInstanceTopics {
    pub instance_id: i32,
    pub topic: String,
}

#[derive(Queryable)]
pub struct Descendants {
    pub id: i32,
    pub word_id: i32,
    pub depth: Option<i32>,
    pub text: Option<String>,
}

#[derive(Insertable)]
#[table_name = "descendants"]
pub struct NewDescendants {
    pub word_id: i32,
    pub depth: Option<i32>,
    pub text: Option<String>,
}

#[derive(Queryable)]
pub struct DescendantsTags {
    pub id: i32,
    pub descendant_id: i32,
    pub tag: String,
}

#[derive(Insertable)]
#[table_name = "descendantstags"]
pub struct NewDescendantsTags {
    pub descendant_id: i32,
    pub tag: String,
}

#[derive(Queryable)]
pub struct DescendantsTemplates {
    pub id: i32,
    pub descendant_id: i32,
    pub template_id: i32,
}

#[derive(Insertable)]
#[table_name = "descendantstemplates"]
pub struct NewDescendantsTemplates {
    pub descendant_id: i32,
    pub template_id: i32,
}

#[derive(Queryable)]
pub struct RelatedRuby {
    pub id: i32,
    pub related_id: i32,
    pub ruby: String,
}

#[derive(Insertable)]
#[table_name = "relatedruby"]
pub struct NewRelatedRuby {
    pub related_id: i32,
    pub ruby: String,
}

#[derive(Queryable)]
pub struct RelatedLinks {
    pub id: i32,
    pub related_id: i32,
    pub links: String,
}

#[derive(Insertable)]
#[table_name = "senseslinks"]
pub struct NewSensesLinks {
    pub sense_id: i32,
    pub links: String,
}

#[derive(Insertable)]
#[table_name = "sensestags"]
pub struct NewSenseTags {
    pub sense_id: i32,
    pub tag: String,
}

#[derive(Queryable)]
pub struct RelatedTags {
    pub id: i32,
    pub descendant_id: i32,
    pub tag: String,
}

#[derive(Insertable)]
#[table_name = "relatedtags"]
pub struct NewRelatedTags {
    pub related_id: i32,
    pub tag: String,
}

#[derive(Queryable)]
pub struct RelatedTopics {
    pub id: i32,
    pub related_id: i32,
    pub topic: String,
}

#[derive(Insertable)]
#[table_name = "relatedtopics"]
pub struct NewRelatedTopics {
    pub related_id: i32,
    pub topic: String,
}


#[derive(Queryable)]
pub struct RelatedUrls {
    pub id: i32,
    pub related_id: i32,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "relatedurls"]
pub struct NewRelatedUrls {
    pub related_id: i32,
    pub url: String,
}

#[derive(Queryable)]
pub struct HeadTemplates {
    pub id: i32,
    pub word_id: i32,
    pub template_id: i32,
}

#[derive(Insertable)]
#[table_name = "headtemplates"]
pub struct NewHeadTemplates {
    pub word_id: i32,
    pub template_id: i32,
}

#[derive(Queryable)]
pub struct SensesTopics {
    pub id: i32,
    pub sense_id: i32,
    pub topic: String,
}

#[derive(Insertable)]
#[table_name = "sensestopics"]
pub struct NewSensesTopics {
    pub sense_id: i32,
    pub topic: String,
}

#[derive(Queryable)]
pub struct ExamplesRuby {
    pub id: i32,
    pub example_id: i32,
    pub ruby: String,
}

#[derive(Insertable)]
#[table_name = "examplesruby"]
pub struct NewExamplesRuby {
    pub example_id: i32,
    pub ruby: String,
}

#[derive(Queryable)]
pub struct GenericVectorField {
    pub id: i32,
    pub word_id: i32,
    pub field_value: String,
}

#[derive(Insertable)]
#[table_name = "genericvectorfield"]
pub struct NewGenericVectorField {
    pub word_id: i32,
    pub field_value: String,
}
