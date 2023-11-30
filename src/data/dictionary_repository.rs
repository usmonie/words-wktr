use diesel::{pg::PgConnection, r2d2::{ConnectionManager, Pool}, RunQueryDsl};
use diesel::associations::HasTable;
use diesel::r2d2::PooledConnection;
use crate::data::models::*;
use crate::domain::Error;
use crate::domain::models::{Category, Related, Relation, Word};
use crate::domain::use_cases::DictionaryRepository;
use crate::schema::categories::dsl::categories;
use crate::schema::forms::dsl::forms;
use crate::schema::formstags::dsl::formstags;
use crate::schema::hyphenations::dsl::hyphenations;
use crate::schema::related::dsl::related;
use crate::schema::relatedruby::dsl::relatedruby;
use crate::schema::relatedtags::dsl::relatedtags;
use crate::schema::relatedtopics::dsl::relatedtopics;
use crate::schema::relatedurls::dsl::relatedurls;
use crate::schema::senseabbreviationslink::dsl::senseabbreviationslink;
use crate::schema::sensealtoflink::dsl::sensealtoflink;
use crate::schema::senseantonymslink::dsl::senseantonymslink;
use crate::schema::sensecategorieslink::dsl::sensecategorieslink;
use crate::schema::sensecoordinatetermslink::dsl::sensecoordinatetermslink;
use crate::schema::sensederivedlink::dsl::sensederivedlink;
use crate::schema::senseformoflink::dsl::senseformoflink;
use crate::schema::senseholonymslink::dsl::senseholonymslink;
use crate::schema::sensehypernymslink::dsl::sensehypernymslink;
use crate::schema::sensehyponymslink::dsl::sensehyponymslink;
use crate::schema::sensemeronymslink::dsl::sensemeronymslink;
use crate::schema::senseproverbslink::dsl::senseproverbslink;
use crate::schema::senserelatedlink::dsl::senserelatedlink;
use crate::schema::senses::dsl::senses;
use crate::schema::senseslinks::dsl::senseslinks;
use crate::schema::sensestags::dsl::sensestags;
use crate::schema::sensesynonymlink::dsl::sensesynonymlink;
use crate::schema::sensetroponymslink::dsl::sensetroponymslink;
use crate::schema::sounds::dsl::sounds;
use crate::schema::soundtags::dsl::soundtags;
use crate::schema::templates::dsl::templates;
use crate::schema::wordabbreviationslink::dsl::wordabbreviationslink;
use crate::schema::wordaltoflink::dsl::wordaltoflink;
use crate::schema::wordantonymslink::dsl::wordantonymslink;
use crate::schema::wordcoordinatetermslink::dsl::wordcoordinatetermslink;
use crate::schema::wordderivedlink::dsl::wordderivedlink;
use crate::schema::wordformoflink::dsl::wordformoflink;
use crate::schema::wordholonymslink::dsl::wordholonymslink;
use crate::schema::wordhypernymslink::dsl::wordhypernymslink;
use crate::schema::wordhyponymslink::dsl::wordhyponymslink;
use crate::schema::wordmeronymslink::dsl::wordmeronymslink;
use crate::schema::wordproverbslink::dsl::wordproverbslink;
use crate::schema::wordrelatedlink::dsl::wordrelatedlink;
use crate::schema::words::dsl::words;
use crate::schema::wordsynonymlink::dsl::wordsynonymlink;
use crate::schema::wordtroponymslink::dsl::wordtroponymslink;

pub struct DieselDictionaryRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselDictionaryRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to crate database pool");

        Self { pool }
    }

    fn store_related<F>(&self, items: Vec<Related>, mut add_link: F)
        where
            F: FnMut(i32)
    {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        for item in items {
            let insertable = NewRelated {
                roman: item.roman,
                alt: item.alt,
                english: item.english,
                sense: item.sense,
                word: item.word,
                source: item.source,
                taxonomic: item.taxonomic,
                qualifier: item.qualifier,
                extra: item.extra,
            };

            let inserted: i32 = diesel::insert_into(related::table())
                .values(insertable)
                .returning(crate::schema::related::id)
                .get_result::<i32>(&mut conn)
                .expect("error while storing relations");

            add_link(inserted);
            let insertable_tags: Vec<NewRelatedTags> = item.tags.into_iter().map(|tag| {
                NewRelatedTags { related_id: inserted, tag }
            }).collect();

            let insertable_topics: Vec<NewRelatedTopics> = item.topics.into_iter().map(|topic| {
                NewRelatedTopics { related_id: inserted, topic }
            }).collect();

            let insertable_urls: Vec<NewRelatedUrls> = item.urls.into_iter().map(|url| {
                NewRelatedUrls { related_id: inserted, url }
            }).collect();

            let insertable_ruby: Vec<NewRelatedRuby> = item.ruby.into_iter().map(|rubies| {
                NewRelatedRuby { related_id: inserted, ruby: serde_json::to_string(&rubies).expect("") }
            }).collect();

            diesel::insert_into(relatedtags)
                .values(insertable_tags)
                .execute(&mut conn)
                .expect("error while storing relations tags");

            diesel::insert_into(relatedtopics)
                .values(insertable_topics)
                .execute(&mut conn)
                .expect("error while storing relations topics");

            diesel::insert_into(relatedurls)
                .values(insertable_urls)
                .execute(&mut conn)
                .expect("error while storing relations urls");

            diesel::insert_into(relatedruby)
                .values(insertable_ruby)
                .execute(&mut conn)
                .expect("error while storing relations rubies");
        }
    }

    fn store_categories<F>(&self, categories_dom: Vec<Category>, mut add_link: F)
        where
            F: FnMut(i32) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        for item in categories_dom {
            let new_category = NewCategories {
                langcode: item.langcode,
                orig: item.orig,
                kind: item.kind,
                name: item.name,
                source: item.source,
            };

            let inserted: i32 = diesel::insert_into(categories::table())
                .values(new_category)
                .returning(crate::schema::categories::id)
                .get_result(&mut conn)
                .expect("error while storing relations");

            add_link(inserted)
        }
    }

    fn store_word_relations(
        &mut self,
        item: &dyn Relation,
        word_id: i32,
    ) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        self.store_related(item.derived(), |related_id| {
            diesel::insert_into(wordderivedlink)
                .values(NewWordDerivedLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.abbreviations(), |related_id| {
            diesel::insert_into(wordabbreviationslink)
                .values(NewWordAbbreviationsLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.alt_of(), |related_id| {
            diesel::insert_into(wordaltoflink)
                .values(NewWordAltOfLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.related(), |related_id| {
            diesel::insert_into(wordrelatedlink)
                .values(NewWordRelatedLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.antonyms(), |related_id| {
            diesel::insert_into(wordantonymslink)
                .values(NewWordAntonymsLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.coordinate_terms(), |related_id| {
            diesel::insert_into(wordcoordinatetermslink)
                .values(NewWordCoordinateTermsLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.synonyms(), |related_id| {
            diesel::insert_into(wordsynonymlink)
                .values(NewWordSynonymLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.form_of(), |related_id| {
            diesel::insert_into(wordformoflink)
                .values(NewWordFormOfLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.proverbs(), |related_id| {
            diesel::insert_into(wordproverbslink)
                .values(NewWordProverbLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.hyponyms(), |related_id| {
            diesel::insert_into(wordhyponymslink)
                .values(NewWordHyponymsLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.hypernyms(), |related_id| {
            diesel::insert_into(wordhypernymslink)
                .values(NewWordHypernymsLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.holonyms(), |related_id| {
            diesel::insert_into(wordholonymslink)
                .values(NewWordHolonymsLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.meronyms(), |related_id| {
            diesel::insert_into(wordmeronymslink)
                .values(NewWordMeronymsLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.troponyms(), |related_id| {
            diesel::insert_into(wordtroponymslink)
                .values(NewWordTroponymsLink {
                    word_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });
    }

    fn store_sense_relations(
        &mut self,
        item: &dyn Relation,
        sense_id: i32,
    ) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        self.store_related(item.derived(), |related_id| {
            diesel::insert_into(sensederivedlink)
                .values(NewSenseDerivedLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.abbreviations(), |related_id| {
            diesel::insert_into(senseabbreviationslink)
                .values(NewSenseAbbreviationsLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.alt_of(), |related_id| {
            diesel::insert_into(sensealtoflink)
                .values(NewSenseAltOfLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.related(), |related_id| {
            diesel::insert_into(senserelatedlink)
                .values(NewSenseRelatedLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.antonyms(), |related_id| {
            diesel::insert_into(senseantonymslink)
                .values(NewSenseAntonymsLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.coordinate_terms(), |related_id| {
            diesel::insert_into(sensecoordinatetermslink)
                .values(NewSenseCoordinateTermsLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.synonyms(), |related_id| {
            diesel::insert_into(sensesynonymlink)
                .values(NewSenseSynonymLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.form_of(), |related_id| {
            diesel::insert_into(senseformoflink)
                .values(NewSenseFormOfLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.proverbs(), |related_id| {
            diesel::insert_into(senseproverbslink)
                .values(NewSenseProverbLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.hyponyms(), |related_id| {
            diesel::insert_into(sensehyponymslink)
                .values(NewSenseHyponymsLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.hypernyms(), |related_id| {
            diesel::insert_into(sensehypernymslink)
                .values(NewSenseHypernymsLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.holonyms(), |related_id| {
            diesel::insert_into(senseholonymslink)
                .values(NewSenseHolonymsLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.meronyms(), |related_id| {
            diesel::insert_into(sensemeronymslink)
                .values(NewSenseMeronymsLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });

        self.store_related(item.troponyms(), |related_id| {
            diesel::insert_into(sensetroponymslink::table())
                .values(NewSenseTroponymsLink {
                    sense_id,
                    related_id,
                })
                .execute(&mut conn)
                .expect("");
        });
    }
}

#[async_trait::async_trait]
impl DictionaryRepository for DieselDictionaryRepository {
    async fn bulk_insert(&mut self, items: Vec<Word>) -> Result<(), Error> {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        for item in items {
            let new_word = NewWord {
                word: item.word.clone(),
                pos: item.pos.clone(),
                source: item.source.clone(),
                lang_code: item.lang_code.clone(),
                lang: item.lang.clone(),
                original_title: item.original_title.clone(),
                etymology_number: item.etymology_number.clone(),
                etymology_text: item.etymology_text.clone(),
            };

            let word_id = diesel::insert_into(words::table())
                .values(new_word)
                .returning(crate::schema::words::id)
                .get_result(&mut conn)
                .expect("");

            self.store_word_relations(&item, word_id);

            for f in item.forms {
                let new_form = NewForm {
                    word_id,
                    form: f.form,
                    head_nr: f.head_nr,
                    roman: f.roman,
                    source: f.source,
                    ipa: f.ipa,
                };

                let form_id = diesel::insert_into(forms::table())
                    .values(new_form)
                    .returning(crate::schema::forms::id)
                    .get_result(&mut conn)
                    .expect("");

                for tag in f.tags {
                    let new_tag = NewFormsTags { forms_id: form_id, tag };
                    diesel::insert_into(formstags::table())
                        .values(new_tag)
                        .execute(&mut conn)
                        .expect("");
                }
            }
            for template in item.etymology_templates {
                let new_template = NewTemplate {
                    word_id,
                    args: serde_json::to_string(&template.args).unwrap(),
                    name: template.name,
                    expansion: template.expansion,
                };
                diesel::insert_into(templates::table())
                    .values(new_template)
                    .execute(&mut conn)
                    .expect("");
            }

            for sound in item.sounds {
                let new_sound = NewSound {
                    word_id,
                    ipa: sound.ipa,
                    enpr: sound.enpr,
                    audio: sound.audio,
                    text: sound.text,
                    ogg_url: sound.ogg_url,
                    mp3_url: sound.mp3_url,
                    homophone: sound.homophone,
                    rhymes: sound.rhymes,
                    note: sound.note,
                    other: sound.other,
                    audio_ipa: sound.audio_ipa,
                    form: sound.form,
                    zh_pron: sound.zh_pron,
                };
                let sound_id = diesel::insert_into(sounds::table())
                    .values(new_sound)
                    .returning(crate::schema::sounds::id)
                    .get_result(&mut conn)
                    .expect("");

                for tag in sound.tags {
                    let new_sound_tag = NewSoundTags {
                        sound_id,
                        tag,
                    };

                    diesel::insert_into(soundtags::table())
                        .values(new_sound_tag)
                        .execute(&mut conn)
                        .expect("");
                }
            }
            for hyphenation_item in item.hyphenation {
                let new_hyphen = NewHyphenation {
                    word_id,
                    hyphenation: hyphenation_item,
                };
                diesel::insert_into(hyphenations::table())
                    .values(new_hyphen)
                    .execute(&mut conn)
                    .expect("");
            }

            for sense in item.senses {
                let new_sense = NewSense {
                    word_id,
                    sense_id: sense.id.clone(),
                    head_nr: sense.head_nr.clone(),
                    taxonomic: sense.taxonomic.clone(),
                    qualifier: sense.qualifier.clone(),
                    glosses: Some(serde_json::to_string(&sense.glosses).expect("")),
                    raw_glosses: Some(serde_json::to_string(&sense.raw_glosses).expect("")),
                };

                let sense_id = diesel::insert_into(senses::table())
                    .values(new_sense)
                    .returning(crate::schema::senses::id)
                    .get_result(&mut conn)
                    .expect("");

                self.store_sense_relations(&sense, sense_id);

                for link in sense.links {
                    let new_link = NewSensesLinks {
                        sense_id,
                        links: serde_json::to_string(&link).expect(""),
                    };

                    diesel::insert_into(senseslinks)
                        .values(new_link)
                        .execute(&mut conn)
                        .expect("");
                }

                for sense_tag in sense.tags {
                    let new_sense_tag = NewSenseTags {
                        sense_id,
                        tag: sense_tag,
                    };
                    diesel::insert_into(sensestags)
                        .values(new_sense_tag)
                        .execute(&mut conn)
                        .expect("");
                }

                self.store_categories(sense.categories, |category_id| {
                    diesel::insert_into(sensecategorieslink::table())
                        .values(NewSenseCategoriesLink {
                            sense_id,
                            category_id,
                        })
                        .execute(&mut conn)
                        .expect("");
                });

            }
        }
        Ok(())
    }

    async fn find(&self, query: &str) -> Result<Option<Vec<Word>>, Error> {

        Ok(Some(vec![]))
    }
}