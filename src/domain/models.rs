use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    pub word: String,
    pub pos: String,
    pub forms: Vec<Form>,
    pub etymology_number: Option<i32>,
    pub etymology_text: Option<String>,
    pub etymology_templates: Vec<EtymologyTemplate>,
    pub lang: String,
    pub lang_code: String,
    pub sounds: Vec<Sound>,
    pub hyphenation: Vec<String>,
    pub derived: Vec<DerivedWord>,
    pub senses: Vec<Sense>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Form {
    pub form: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EtymologyTemplate {
    pub name: String,
    pub args: HashMap<String, String>,
    pub expansion: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sound {
    pub ipa: Option<String>,
    pub enpr: Option<String>,
    pub audio: Option<String>,
    pub text: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ogg_url: Option<String>,
    pub mp3_url: Option<String>,
    pub homophone: Option<String>,
    pub rhymes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DerivedWord {
    pub word: String,
    pub _dis1: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sense {
    pub links: Option<Vec<Vec<String>>>,
    pub raw_glosses: Option<Vec<String>>,
    pub glosses: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub id: String,
    pub categories: Option<Vec<Category>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub name: String,
    pub kind: String,
    pub parents: Option<Vec<String>>,
    pub source: String,
    pub orig: Option<String>,
    pub langcode: Option<String>,
    pub _dis: Option<String>,
}
