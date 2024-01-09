use diesel::{Insertable, Selectable};
use diesel::Queryable;
use crate::schema::*;
use diesel::prelude::*;
use crate::data::db::db_models::*;
use crate::data::db::models::*;

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "related_tags"]
#[diesel(belongs_to(Related))]
pub struct RelatedTagLink {
    pub related_id: i32,
    pub tag: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "related_urls"]
#[diesel(belongs_to(Related))]
pub struct RelatedUrlLink {
    pub related_id: i32,
    pub url: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "related_topics"]
#[diesel(belongs_to(Related))]
pub struct RelatedTopicLink {
    pub related_id: i32,
    pub topic: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "related_ruby"]
#[diesel(belongs_to(Related))]
pub struct RelatedRubyLink {
    pub related_id: i32,
    pub ruby: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "forms_ruby"]
#[diesel(belongs_to(Form, foreign_key = forms_id))]
pub struct FormRubyLink {
    pub forms_id: i32,
    pub ruby: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "forms_tags"]
#[diesel(belongs_to(Form, foreign_key = forms_id))]
pub struct FormTagLink {
    pub forms_id: i32,
    pub tag: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "translation_topics"]
#[diesel(belongs_to(Translation))]
pub struct TranslationTopicLink {
    pub translation_id: i32,
    pub topic: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "translation_tags"]
#[diesel(belongs_to(Translation))]
pub struct TranslationTagLink {
    pub translation_id: i32,
    pub tag: String,
}

#[derive(Queryable, Selectable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "translations_words_link"]
// #[diesel(belongs_to(Translation))]
#[diesel(belongs_to(Word))]
pub struct TranslationWordLink {
    pub id: i32,
    pub translation_id: i32,
    pub word_id: i32,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "sense_tags"]
#[diesel(belongs_to(Sense))]
pub struct SenseTagLink {
    pub sense_id: i32,
    pub tag: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "sense_links"]
#[diesel(belongs_to(Sense))]
pub struct SenseLinksLink {
    pub sense_id: i32,
    pub link: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "sense_topics"]
#[diesel(belongs_to(Sense))]
pub struct SenseTopicLink {
    pub sense_id: i32,
    pub topic: String,
}

// #[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
// #[table_name = "soundlinks]
// pub struct SoundLinksLink {
//     pub sound_id: i32,
//     pub link: String,
// }

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "sound_topics"]
#[diesel(belongs_to(Sound))]
pub struct SoundTopicLink {
    pub sound_id: i32,
    pub topic: String,
}

#[derive(Queryable, Associations, Selectable, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "sound_tags"]
#[diesel(belongs_to(Sound))]
pub struct SoundTagLink {
    pub sound_id: i32,
    pub tag: String,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "senserelatedlink"]
#[diesel(belongs_to(Sense))]
#[diesel(belongs_to(Related))]
pub struct SenseRelatedLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Queryable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Word))]
#[diesel(belongs_to(Related))]
#[table_name = "wordrelatedlink"]
pub struct WordRelatedLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordcompoundoflink"]
pub struct NewWordCompoundOfLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sensecompoundoflink"]
pub struct NewSensesSynonymLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordsynonymlink"]
pub struct NewWordSynonymLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sensesynonymlink"]
pub struct NewSenseSynonymLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordtroponymslink"]
pub struct NewWordTroponymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sensetroponymslink"]
pub struct NewSenseTroponymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordantonymslink"]
pub struct NewWordAntonymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "senseantonymslink"]
pub struct NewSenseAntonymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordcoordinatetermslink"]
pub struct NewWordCoordinateTermsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sensecoordinatetermslink"]
pub struct NewSenseCoordinateTermsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordmeronymslink"]
pub struct NewWordMeronymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sensemeronymslink"]
pub struct NewSenseMeronymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordrelatedlink"]
pub struct NewWordRelatedLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "senserelatedlink"]
pub struct NewSenseRelatedLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordderivedlink"]
pub struct NewWordDerivedLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sensederivedlink"]
pub struct NewSenseDerivedLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordholonymslink"]
pub struct NewWordHolonymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "senseholonymslink"]
pub struct NewSenseHolonymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordhypernymslink"]
pub struct NewWordHypernymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sensehypernymslink"]
pub struct NewSenseHypernymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordhyponymslink"]
pub struct NewWordHyponymsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sensehyponymslink"]
pub struct NewSenseHyponymsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "wordcategorieslink"]
#[diesel(belongs_to(Word))]
#[diesel(belongs_to(Category, foreign_key = category_id))]
pub struct WordCategoriesLink {
    pub word_id: i32,
    pub category_id: i32,
}

#[derive(Queryable, Associations, Debug, PartialEq, Insertable, AsChangeset)]
#[table_name = "sensecategorieslink"]
#[diesel(belongs_to(Sense))]
#[diesel(belongs_to(Category, foreign_key = category_id))]
pub struct NewSenseCategoriesLink {
    pub sense_id: i32,
    pub category_id: i32,
}


#[derive(Insertable, AsChangeset)]
#[table_name = "wordabbreviationslink"]
pub struct NewWordAbbreviationsLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "senseabbreviationslink"]
pub struct NewSenseAbbreviationsLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordformoflink"]
pub struct NewWordFormOfLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "senseformoflink"]
pub struct NewSenseFormOfLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordproverbslink"]
pub struct NewWordProverbLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "senseproverbslink"]
pub struct NewSenseProverbLink {
    pub sense_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "wordaltoflink"]
pub struct NewWordAltOfLink {
    pub word_id: i32,
    pub related_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "sensealtoflink"]
pub struct NewSenseAltOfLink {
    pub sense_id: i32,
    pub related_id: i32,
}