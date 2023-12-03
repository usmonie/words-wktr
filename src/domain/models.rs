use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub trait Relation {
    fn abbreviations(&self) -> Vec<Related>;
    fn alt_of(&self) -> Vec<Related>;
    fn antonyms(&self) -> Vec<Related>;
    fn derived(&self) -> Vec<Related>;
    fn coordinate_terms(&self) -> Vec<Related>;
    fn synonyms(&self) -> Vec<Related>;
    fn troponyms(&self) -> Vec<Related>;
    fn proverbs(&self) -> Vec<Related>;
    fn related(&self) -> Vec<Related>;
    fn form_of(&self) -> Vec<Related>;
    fn meronyms(&self) -> Vec<Related>;
    fn holonyms(&self) -> Vec<Related>;
    fn hyponyms(&self) -> Vec<Related>;
    fn hypernyms(&self) -> Vec<Related>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Word {
    #[serde(rename = "word")]
    pub word: String,

    #[serde(rename = "pos")]
    pub pos: String,

    #[serde(rename = "lang_code")]
    pub lang_code: String,

    #[serde(rename = "lang")]
    pub lang: String,

    #[serde(rename = "etymology_number")]
    pub etymology_number: Option<i32>,

    #[serde(rename = "etymology_templates")]
    #[serde(default)]
    pub etymology_templates: Vec<Template>,

    #[serde(rename = "etymology_text")]
    pub etymology_text: Option<String>,

    #[serde(rename = "inflection_templates")]
    #[serde(default)]
    pub inflection_templates: Vec<InflectionTemplate>,

    #[serde(rename = "translations")]
    #[serde(default)]
    pub translations: Vec<Translation>,

    #[serde(rename = "categories")]
    #[serde(default)]
    pub categories: Vec<Category>,

    #[serde(rename = "wikipedia")]
    #[serde(default)]
    pub wikipedia: Vec<String>,

    #[serde(rename = "hyphenation")]
    #[serde(default)]
    pub hyphenation: Vec<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    pub topics: Vec<String>,

    #[serde(rename = "glosses")]
    #[serde(default)]
    pub glosses: Vec<String>,

    #[serde(rename = "raw_glosses")]
    #[serde(default)]
    pub raw_glosses: Vec<String>,

    #[serde(rename = "forms")]
    #[serde(default)]
    pub forms: Vec<Form>,

    #[serde(rename = "senses")]
    #[serde(default)]
    pub senses: Vec<Sense>,

    #[serde(rename = "instances")]
    #[serde(default)]
    pub instances: Vec<Instance>,

    #[serde(rename = "head_templates")]
    #[serde(default)]
    pub head_templates: Vec<Template>,

    #[serde(rename = "descendants")]
    #[serde(default)]
    pub descendants: Vec<Descendant>,

    #[serde(rename = "sounds")]
    #[serde(default)]
    pub sounds: Vec<Sound>,

    #[serde(rename = "antonyms")]
    #[serde(default)]
    pub antonyms: Vec<Related>,

    #[serde(rename = "hypernyms")]
    #[serde(default)]
    pub hypernyms: Vec<Related>,

    #[serde(rename = "related")]
    #[serde(default)]
    pub related: Vec<Related>,

    #[serde(rename = "derived")]
    #[serde(default)]
    pub derived: Vec<Related>,

    #[serde(rename = "abbreviations")]
    #[serde(default)]
    pub abbreviations: Vec<Related>,

    #[serde(rename = "synonyms")]
    #[serde(default)]
    pub synonyms: Vec<Related>,

    #[serde(rename = "troponyms")]
    #[serde(default)]
    pub troponyms: Vec<Related>,

    #[serde(rename = "form_of")]
    #[serde(default)]
    pub form_of: Vec<Related>,

    #[serde(rename = "hyponyms")]
    #[serde(default)]
    pub hyponyms: Vec<Related>,

    #[serde(rename = "alt_of")]
    #[serde(default)]
    pub alt_of: Vec<Related>,

    #[serde(rename = "holonyms")]
    #[serde(default)]
    pub holonyms: Vec<Related>,

    #[serde(rename = "coordinate_terms")]
    #[serde(default)]
    pub coordinate_terms: Vec<Related>,

    #[serde(rename = "meronyms")]
    #[serde(default)]
    pub meronyms: Vec<Related>,

    #[serde(rename = "proverbs")]
    #[serde(default)]
    pub proverbs: Vec<Related>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Related {
    #[serde(rename = "urls")]
    #[serde(default)]
    pub urls: Vec<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    pub topics: Vec<String>,

    #[serde(rename = "roman")]
    pub roman: Option<String>,

    #[serde(rename = "alt")]
    pub alt: Option<String>,

    #[serde(rename = "english")]
    pub english: Option<String>,

    #[serde(rename = "sense")]
    pub sense: Option<String>,

    #[serde(rename = "word")]
    pub word: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(rename = "ruby")]
    #[serde(default)]
    pub ruby: Vec<Vec<String>>,

    #[serde(rename = "taxonomic")]
    pub taxonomic: Option<String>,

    #[serde(rename = "qualifier")]
    pub qualifier: Option<String>,

    #[serde(rename = "extra")]
    pub extra: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    #[serde(rename = "langcode")]
    pub langcode: Option<String>,

    #[serde(rename = "orig")]
    pub orig: Option<String>,

    #[serde(rename = "kind")]
    pub kind: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "parents")]
    #[serde(default)]
    pub parents: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Descendant {
    #[serde(rename = "depth")]
    pub depth: Option<i32>,

    #[serde(rename = "templates")]
    #[serde(default)]
    pub templates: Vec<Template>,

    #[serde(rename = "text")]
    pub text: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    #[serde(rename = "args")]
    #[serde(default)]
    pub args: HashMap<String, String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "expansion")]
    pub expansion: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Form {
    #[serde(rename = "form")]
    pub form: Option<String>,

    #[serde(rename = "head_nr")]
    pub head_nr: Option<i32>,

    #[serde(rename = "topics")]
    #[serde(default)]
    pub topics: Vec<String>,

    #[serde(rename = "roman")]
    pub roman: Option<String>,

    #[serde(rename = "ipa")]
    pub ipa: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(rename = "ruby")]
    #[serde(default)]
    pub ruby: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InflectionTemplate {
    #[serde(rename = "args")]
    #[serde(default)]
    pub args: HashMap<String, String>,

    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Instance {
    #[serde(rename = "sense")]
    pub sense: Option<String>,

    #[serde(rename = "word")]
    pub word: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    pub topics: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sense {
    #[serde(rename = "senseid")]
    #[serde(default)]
    pub senseid: Vec<String>,

    #[serde(rename = "proverbs")]
    #[serde(default)]
    pub proverbs: Vec<Related>,

    #[serde(rename = "alt_of")]
    #[serde(default)]
    pub alt_of: Vec<Related>,

    #[serde(rename = "instances")]
    #[serde(default)]
    pub instances: Vec<Instance>,

    #[serde(rename = "glosses")]
    #[serde(default)]
    pub glosses: Vec<String>,

    #[serde(rename = "coordinate_terms")]
    #[serde(default)]
    pub coordinate_terms: Vec<Related>,

    #[serde(rename = "meronyms")]
    #[serde(default)]
    pub meronyms: Vec<Related>,

    #[serde(rename = "compound_of")]
    #[serde(default)]
    pub compound_of: Vec<Related>,

    #[serde(rename = "holonyms")]
    #[serde(default)]
    pub holonyms: Vec<Related>,

    #[serde(rename = "related")]
    #[serde(default)]
    pub related: Vec<Related>,

    #[serde(rename = "abbreviations")]
    #[serde(default)]
    pub abbreviations: Vec<Related>,

    #[serde(rename = "hypernyms")]
    #[serde(default)]
    pub hypernyms: Vec<Related>,

    #[serde(rename = "translations")]
    #[serde(default)]
    pub translations: Vec<Translation>,

    #[serde(rename = "antonyms")]
    #[serde(default)]
    pub antonyms: Vec<Related>,

    #[serde(rename = "links")]
    #[serde(default)]
    pub links: Vec<Vec<String>>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "categories")]
    #[serde(default)]
    pub categories: Vec<Category>,

    #[serde(rename = "wikipedia")]
    #[serde(default)]
    pub wikipedia: Vec<String>,

    #[serde(rename = "derived")]
    #[serde(default)]
    pub derived: Vec<Related>,

    #[serde(rename = "head_nr")]
    pub head_nr: Option<i32>,

    #[serde(rename = "synonyms")]
    #[serde(default)]
    pub synonyms: Vec<Related>,

    #[serde(rename = "topics")]
    #[serde(default)]
    pub topics: Vec<String>,

    #[serde(rename = "raw_glosses")]
    #[serde(default)]
    pub raw_glosses: Vec<String>,

    #[serde(rename = "troponyms")]
    #[serde(default)]
    pub troponyms: Vec<Related>,

    #[serde(rename = "form_of")]
    #[serde(default)]
    pub form_of: Vec<Related>,

    #[serde(rename = "taxonomic")]
    pub taxonomic: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(rename = "examples")]
    #[serde(default)]
    pub examples: Vec<Example>,

    #[serde(rename = "qualifier")]
    pub qualifier: Option<String>,

    #[serde(rename = "hyponyms")]
    #[serde(default)]
    pub hyponyms: Vec<Related>,

    #[serde(rename = "wikidata")]
    #[serde(default)]
    pub wikidata: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Example {
    #[serde(rename = "note")]
    pub note: Option<String>,

    #[serde(rename = "ref")]
    pub example_ref: Option<String>,

    #[serde(rename = "roman")]
    pub roman: Option<String>,

    #[serde(rename = "english")]
    pub english: Option<String>,

    #[serde(rename = "text")]
    pub text: Option<String>,

    #[serde(rename = "type")]
    pub example_type: Option<String>,

    #[serde(rename = "ruby")]
    #[serde(default)]
    pub ruby: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Translation {
    #[serde(rename = "note")]
    pub note: Option<String>,

    #[serde(rename = "code")]
    pub code: Option<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    pub topics: Vec<String>,

    #[serde(rename = "roman")]
    pub roman: Option<String>,

    #[serde(rename = "alt")]
    pub alt: Option<String>,

    #[serde(rename = "english")]
    pub english: Option<String>,

    #[serde(rename = "sense")]
    pub sense: Option<String>,

    #[serde(rename = "lang")]
    pub lang: Option<String>,

    #[serde(rename = "word")]
    pub word: Option<String>,

    #[serde(rename = "taxonomic")]
    pub taxonomic: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sound {
    #[serde(rename = "mp3_url")]
    pub mp3_url: Option<String>,

    #[serde(rename = "note")]
    pub note: Option<String>,

    #[serde(rename = "rhymes")]
    pub rhymes: Option<String>,

    #[serde(rename = "other")]
    pub other: Option<String>,

    #[serde(rename = "enpr")]
    pub enpr: Option<String>,

    #[serde(rename = "audio-ipa")]
    pub audio_ipa: Option<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    pub topics: Vec<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(rename = "ogg_url")]
    pub ogg_url: Option<String>,

    #[serde(rename = "form")]
    pub form: Option<String>,

    #[serde(rename = "ipa")]
    pub ipa: Option<String>,

    #[serde(rename = "audio")]
    pub audio: Option<String>,

    #[serde(rename = "text")]
    pub text: Option<String>,

    #[serde(rename = "homophone")]
    pub homophone: Option<String>,

    #[serde(rename = "zh-pron")]
    pub zh_pron: Option<String>,
}

impl Relation for Word {
    fn abbreviations(&self) -> Vec<Related> {
        return self.abbreviations.clone();
    }

    fn alt_of(&self) -> Vec<Related> {
        return self.alt_of.clone();
    }

    fn antonyms(&self) -> Vec<Related> {
        return self.antonyms.clone();
    }

    fn derived(&self) -> Vec<Related> {
        return self.derived.clone();
    }

    fn coordinate_terms(&self) -> Vec<Related> {
        return self.coordinate_terms.clone();
    }

    fn synonyms(&self) -> Vec<Related> {
        return self.synonyms.clone();
    }

    fn troponyms(&self) -> Vec<Related> {
        return self.troponyms.clone();
    }

    fn proverbs(&self) -> Vec<Related> {
        return self.proverbs.clone();
    }

    fn related(&self) -> Vec<Related> {
        return self.related.clone();
    }

    fn form_of(&self) -> Vec<Related> {
        return self.form_of.clone();
    }

    fn meronyms(&self) -> Vec<Related> {
        return self.meronyms.clone();
    }

    fn holonyms(&self) -> Vec<Related> {
        return self.holonyms.clone();
    }

    fn hyponyms(&self) -> Vec<Related> {
        return self.hyponyms.clone();
    }

    fn hypernyms(&self) -> Vec<Related> {
        return self.hypernyms.clone();
    }
}
impl Relation for Sense {
    fn abbreviations(&self) -> Vec<Related> {
        return self.abbreviations.clone();
    }

    fn alt_of(&self) -> Vec<Related> {
        return self.alt_of.clone();
    }

    fn antonyms(&self) -> Vec<Related> {
        return self.antonyms.clone();
    }

    fn derived(&self) -> Vec<Related> {
        return self.derived.clone();
    }

    fn coordinate_terms(&self) -> Vec<Related> {
        return self.coordinate_terms.clone();
    }

    fn synonyms(&self) -> Vec<Related> {
        return self.synonyms.clone();
    }

    fn troponyms(&self) -> Vec<Related> {
        return self.troponyms.clone();
    }

    fn proverbs(&self) -> Vec<Related> {
        return self.proverbs.clone();
    }

    fn related(&self) -> Vec<Related> {
        return self.related.clone();
    }

    fn form_of(&self) -> Vec<Related> {
        return self.form_of.clone();
    }

    fn meronyms(&self) -> Vec<Related> {
        return self.meronyms.clone();
    }

    fn holonyms(&self) -> Vec<Related> {
        return self.holonyms.clone();
    }

    fn hyponyms(&self) -> Vec<Related> {
        return self.hyponyms.clone();
    }

    fn hypernyms(&self) -> Vec<Related> {
        return self.hypernyms.clone();
    }
}