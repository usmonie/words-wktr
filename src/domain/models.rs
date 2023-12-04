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

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub etymology_number: Option<i32>,

    #[serde(rename = "etymology_templates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub etymology_templates: Vec<Template>,

    #[serde(rename = "etymology_text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub etymology_text: Option<String>,

    #[serde(rename = "inflection_templates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inflection_templates: Vec<InflectionTemplate>,

    #[serde(rename = "translations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<Translation>,

    #[serde(rename = "categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<Category>,

    #[serde(rename = "wikipedia")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub wikipedia: Vec<String>,

    #[serde(rename = "hyphenation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hyphenation: Vec<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,

    #[serde(rename = "glosses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub glosses: Vec<String>,

    #[serde(rename = "raw_glosses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub raw_glosses: Vec<String>,

    #[serde(rename = "forms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forms: Vec<Form>,

    #[serde(rename = "senses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub senses: Vec<Sense>,

    #[serde(rename = "instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<Instance>,

    #[serde(rename = "head_templates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub head_templates: Vec<Template>,

    #[serde(rename = "descendants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub descendants: Vec<Descendant>,

    #[serde(rename = "sounds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sounds: Vec<Sound>,

    #[serde(rename = "antonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub antonyms: Vec<Related>,

    #[serde(rename = "hypernyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hypernyms: Vec<Related>,

    #[serde(rename = "related")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<Related>,

    #[serde(rename = "derived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derived: Vec<Related>,

    #[serde(rename = "abbreviations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub abbreviations: Vec<Related>,

    #[serde(rename = "synonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<Related>,

    #[serde(rename = "troponyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub troponyms: Vec<Related>,

    #[serde(rename = "form_of")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form_of: Vec<Related>,

    #[serde(rename = "hyponyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hyponyms: Vec<Related>,

    #[serde(rename = "alt_of")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alt_of: Vec<Related>,

    #[serde(rename = "holonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub holonyms: Vec<Related>,

    #[serde(rename = "coordinate_terms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub coordinate_terms: Vec<Related>,

    #[serde(rename = "meronyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meronyms: Vec<Related>,

    #[serde(rename = "proverbs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub proverbs: Vec<Related>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Related {
    #[serde(rename = "urls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,

    #[serde(rename = "roman")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub roman: Option<String>,

    #[serde(rename = "alt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub alt: Option<String>,

    #[serde(rename = "english")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub english: Option<String>,

    #[serde(rename = "sense")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sense: Option<String>,

    #[serde(rename = "word")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub word: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(rename = "ruby")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ruby: Vec<Vec<String>>,

    #[serde(rename = "taxonomic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub taxonomic: Option<String>,

    #[serde(rename = "qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub qualifier: Option<String>,

    #[serde(rename = "extra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub extra: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Category {
    #[serde(rename = "langcode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub langcode: Option<String>,

    #[serde(rename = "orig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub orig: Option<String>,

    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub kind: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,

    #[serde(rename = "parents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Descendant {
    #[serde(rename = "depth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub depth: Option<i32>,

    #[serde(rename = "templates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub templates: Vec<Template>,

    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Template {
    #[serde(rename = "args")]
    #[serde(default)]
    pub args: HashMap<String, String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,

    #[serde(rename = "expansion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub expansion: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Form {
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub form: Option<String>,

    #[serde(rename = "head_nr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub head_nr: Option<i32>,

    #[serde(rename = "topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,

    #[serde(rename = "roman")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub roman: Option<String>,

    #[serde(rename = "ipa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ipa: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(rename = "ruby")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ruby: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InflectionTemplate {
    #[serde(rename = "args")]
    #[serde(default)]
    pub args: HashMap<String, String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Instance {
    #[serde(rename = "sense")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sense: Option<String>,

    #[serde(rename = "word")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub word: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sense {
    #[serde(rename = "senseid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub senseid: Vec<String>,

    #[serde(rename = "proverbs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub proverbs: Vec<Related>,

    #[serde(rename = "alt_of")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alt_of: Vec<Related>,

    #[serde(rename = "instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<Instance>,

    #[serde(rename = "glosses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub glosses: Vec<String>,

    #[serde(rename = "coordinate_terms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub coordinate_terms: Vec<Related>,

    #[serde(rename = "meronyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meronyms: Vec<Related>,

    #[serde(rename = "compound_of")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub compound_of: Vec<Related>,

    #[serde(rename = "holonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub holonyms: Vec<Related>,

    #[serde(rename = "related")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<Related>,

    #[serde(rename = "abbreviations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub abbreviations: Vec<Related>,

    #[serde(rename = "hypernyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hypernyms: Vec<Related>,

    #[serde(rename = "translations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<Translation>,

    #[serde(rename = "antonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub antonyms: Vec<Related>,

    #[serde(rename = "links")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Vec<String>>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,

    #[serde(rename = "categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<Category>,

    #[serde(rename = "wikipedia")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub wikipedia: Vec<String>,

    #[serde(rename = "derived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derived: Vec<Related>,

    #[serde(rename = "head_nr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub head_nr: Option<i32>,

    #[serde(rename = "synonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<Related>,

    #[serde(rename = "topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,

    #[serde(rename = "raw_glosses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub raw_glosses: Vec<String>,

    #[serde(rename = "troponyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub troponyms: Vec<Related>,

    #[serde(rename = "form_of")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form_of: Vec<Related>,

    #[serde(rename = "taxonomic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub taxonomic: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(rename = "examples")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<Example>,

    #[serde(rename = "qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub qualifier: Option<String>,

    #[serde(rename = "hyponyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hyponyms: Vec<Related>,

    #[serde(rename = "wikidata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub wikidata: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Example {
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub note: Option<String>,

    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub example_ref: Option<String>,

    #[serde(rename = "roman")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub roman: Option<String>,

    #[serde(rename = "english")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub english: Option<String>,

    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub example_type: Option<String>,

    #[serde(rename = "ruby")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ruby: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Translation {
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub note: Option<String>,

    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub code: Option<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,

    #[serde(rename = "roman")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub roman: Option<String>,

    #[serde(rename = "alt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub alt: Option<String>,

    #[serde(rename = "english")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub english: Option<String>,

    #[serde(rename = "sense")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sense: Option<String>,

    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lang: Option<String>,

    #[serde(rename = "word")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub word: Option<String>,

    #[serde(rename = "taxonomic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub taxonomic: Option<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sound {
    #[serde(rename = "mp3_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mp3_url: Option<String>,

    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub note: Option<String>,

    #[serde(rename = "rhymes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rhymes: Option<String>,

    #[serde(rename = "other")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub other: Option<String>,

    #[serde(rename = "enpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enpr: Option<String>,

    #[serde(rename = "audio-ipa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub audio_ipa: Option<String>,

    #[serde(rename = "topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,

    #[serde(rename = "tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(rename = "ogg_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ogg_url: Option<String>,

    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub form: Option<String>,

    #[serde(rename = "ipa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ipa: Option<String>,

    #[serde(rename = "audio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub audio: Option<String>,

    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text: Option<String>,

    #[serde(rename = "homophone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub homophone: Option<String>,

    #[serde(rename = "zh-pron")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
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