use std::collections::HashMap;
use crate::domain::use_cases::DictionaryRepository;
use crate::domain::Error;
use diesel::associations::HasTable;
use diesel::{BelongingToDsl, ExpressionMethods, Identifiable, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::data::db::links::*;
use crate::data::db::models::*;
use crate::domain::models::{Category, Relation};
use crate::schema::words::word;
use std::iter::Iterator;
use crate::schema::translations::id;

pub struct DieselDictionaryRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselDictionaryRepository {
    fn map_words(pool: &Pool<ConnectionManager<PgConnection>>, words_db: Vec<Word>) -> Vec<crate::domain::models::Word> {
        let mut results: Vec<crate::domain::models::Word> = Vec::new();
        for word_db in words_db {
            let word_dm: crate::domain::models::Word = Self::collect_word(pool, word_db);
            results.push(word_dm);
        }
        results
    }

    fn collect_word(pool: &Pool<ConnectionManager<PgConnection>>, word_db: Word) -> crate::domain::models::Word {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();

        let templates = Template::belonging_to(&word_db)
            .load::<Template>(&mut conn)
            .unwrap_or_default();
        let templates: Vec<crate::domain::models::Template> = templates.into_iter().map(|template| {
            let args: HashMap<String, String> = match template.args {
                None => { Default::default() }
                Some(arg) => { serde_json::from_str(&arg).unwrap() }
            };

            crate::domain::models::Template {
                args,
                name: template.name,
                expansion: template.expansion,
            }
        }).collect();

        let translations_links = crate::schema::translations_words_link::dsl::translations_words_link::table()
            .filter(crate::schema::translations_words_link::word_id.eq(&word_db.id()))
            .load::<TranslationWordLink>(&mut conn)
            .unwrap_or_default();
        let translations_links = translations_links.into_iter().map(|relation| {
            relation.translation_id
        });

        let translations = crate::schema::translations::dsl::translations::table()
            .filter(id.eq_any(translations_links))
            .load::<Translation>(&mut conn)
            .expect("");

        // let translations = translations.into_iter().map(|translation| {
        //     crate::domain::models::Translation {
        //         note: translation.note,
        //         code: translation.code,
        //         topics: vec![],
        //         roman: translation.roman,
        //         alt: translation.alt,
        //         english: translation.english,
        //         sense: translation.sense,
        //         lang: translation.lang,
        //         word: translation.word,
        //         taxonomic: translation.taxonomic,
        //         tags: vec![],
        //     }
        // }).collect();

        let sounds = Sound::belonging_to(&word_db)
            .load::<Sound>(&mut conn)
            .unwrap_or_default();
        let sounds = sounds.into_iter().map(|sound| {
            // let topics: Vec<SoundTopicLink> = crate::schema::sound_topics::dsl::sound_topics::b
            // let topics: Vec<SoundTopicLink> = SoundTopicLink::belonging_to(&sound)
            //     .select(SoundTopicLink::as_select())
            //     .load(&mut conn)
            //     .unwrap();
            //
            // let topics = topics.into_iter().map(|topic| {
            //     topic.topic
            // }).collect();
            // let tags: Vec<SoundTagLink> = SoundTagLink::belonging_to(&sound)
            //     .select(SoundTagLink::as_select())
            //     .load(&mut conn)
            //     .unwrap();
            //
            // let tags = tags.into_iter().map(|tag| {
            //     tag.tag
            // }).collect();

            crate::domain::models::Sound {
                mp3_url: sound.mp3_url,
                note: sound.note,
                rhymes: sound.rhymes,
                other: sound.other,
                enpr: sound.enpr,
                audio_ipa: sound.audio_ipa,
                topics: vec![],
                tags: vec![],
                ogg_url: sound.ogg_url,
                form: sound.form,
                ipa: sound.ipa,
                audio: sound.audio,
                text: sound.text,
                homophone: sound.homophone,
                zh_pron: sound.zh_pron,
            }
        }).collect();

        let categories_links = crate::schema::wordcategorieslink::dsl::wordcategorieslink::table()
            .filter(crate::schema::wordcategorieslink::word_id.eq(&word_db.id()))
            .load::<WordCategoriesLink>(&mut conn)
            .unwrap_or_default();
        let categories_links = categories_links.into_iter().map(|relation| {
            relation.category_id
        });

        let categories = crate::schema::categories::dsl::categories::table()
            .filter(crate::schema::categories::id.eq_any(categories_links))
            .load::<Categorie>(&mut conn)
            .unwrap_or_default();

        let categories = categories.into_iter().map(|category| {
            Category {
                langcode: category.langcode,
                orig: category.orig,
                kind: category.kind,
                name: category.name,
                parents: vec![],
            }
        }).collect();

        let wikipedia = Wikipedia::belonging_to(&word_db)
            .load::<Wikipedia>(&mut conn)
            .unwrap_or_default();
        let wikipedia: Vec<String> = wikipedia.into_iter().map(|wiki| {
            wiki.wikipedia_link
        }).collect();

        // let hyphenations: Vec<Hyphenation> = Hyphenation::belonging_to(&word_db)
        //     .select(Hyphenation::as_select())
        //     .load(&mut conn)
        //     .unwrap();
        //
        // let hyphenations = hyphenations.into_iter().map(|hyphenation| {
        //     hyphenation.hyphenation
        // }).collect();

        let senses: Vec<Sense> = Sense::belonging_to(&word_db)
            .select(Sense::as_select())
            .load(&mut conn)
            .unwrap();


        let senses = senses.into_iter().map(|sense| {

            let examples: Vec<Example> = Example::belonging_to(&sense)
                .select(Example::as_select())
                .load(&mut conn)
                .unwrap();

            let examples: Vec<crate::domain::models::Example> = examples.into_iter().map(|example| {
                return crate::domain::models::Example {
                    note: example.note,
                    example_ref: example.example_ref,
                    roman: example.roman,
                    english: example.english,
                    text: example.text,
                    example_type: example.example_type,
                    ruby: vec![]
                };
            }).collect();

            crate::domain::models::Sense {
                id: sense.sense_id,
                qualifier: sense.qualifier,
                taxonomic: sense.taxonomic,
                head_nr: sense.head_nr,
                senseid: vec![],
                proverbs: vec![],
                alt_of: vec![],
                instances: vec![],
                glosses: match sense.glosses {
                    None => { vec![] }
                    Some(glosses) => { serde_json::from_str(&glosses).unwrap() }
                },
                coordinate_terms: vec![],
                meronyms: vec![],
                compound_of: vec![],
                holonyms: vec![],
                related: vec![],
                abbreviations: vec![],
                hypernyms: vec![],
                translations: vec![],
                antonyms: vec![],
                links: vec![],
                categories: vec![],
                wikipedia: vec![],
                derived: vec![],
                synonyms: vec![],
                topics: vec![],
                raw_glosses: match sense.raw_glosses {
                    None => { vec![] }
                    Some(raw_glosses) => { serde_json::from_str(&raw_glosses).unwrap() }
                },
                troponyms: vec![],
                form_of: vec![],
                tags: vec![],
                examples: examples,
                hyponyms: vec![],
                wikidata: vec![],
            }
        }).collect();

        // let forms = Form::belonging
        return crate::domain::models::Word {
            word: word_db.word,
            pos: word_db.pos,
            lang_code: word_db.lang_code,
            lang: word_db.lang,
            etymology_number: word_db.etymology_number,
            etymology_templates: templates,
            etymology_text: word_db.etymology_text,
            inflection_templates: vec![],
            translations: vec![],
            categories,
            wikipedia,
            hyphenation: vec![],
            topics: vec![],
            glosses: vec![],
            raw_glosses: vec![],
            forms: vec![],
            senses,
            instances: vec![],
            head_templates: vec![],
            descendants: vec![],
            sounds,
            antonyms: vec![],
            hypernyms: vec![],
            related: vec![],
            derived: vec![],
            abbreviations: vec![],
            synonyms: vec![],
            troponyms: vec![],
            form_of: vec![],
            hyponyms: vec![],
            alt_of: vec![],
            holonyms: vec![],
            coordinate_terms: vec![],
            meronyms: vec![],
            proverbs: vec![],
        };
    }
}

impl DieselDictionaryRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to crate database pool");

        Self { pool }
    }

    fn insert_related<F>(&self, items: Vec<crate::domain::models::Related>, mut add_link: F)
        where
            F: FnMut(i32),
    {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        for item in items {
            let related = NewRelated {
                roman: item.roman,
                alt: item.alt,
                english: item.english,
                sense: item.sense,
                word: item.word,
                taxonomic: item.taxonomic,
                qualifier: item.qualifier,
                extra: item.extra,
            };

            let related_id: i32 = diesel::insert_into(crate::schema::related::dsl::related::table())
                .values(related)
                .returning(crate::schema::related::id)
                .on_conflict_do_nothing()
                .get_result::<i32>(&mut conn)
                .expect("error while storing relations");

            add_link(related_id);

            let mut items = vec![];
            self.insert_tags(&item.tags, |tag| {
                items.push(RelatedTagLink {
                    related_id,
                    tag: tag.to_owned(),
                })
            });
            diesel::insert_into(crate::schema::related_tags::dsl::related_tags::table())
                .values(items)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .expect("insert_related_tags_link");

            let mut items = vec![];
            self.insert_topics(&item.topics, |topic| {
                items.push(RelatedTopicLink {
                    related_id,
                    topic: topic.to_owned(),
                })
            });
            diesel::insert_into(crate::schema::related_topics::dsl::related_topics::table())
                .values(items)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .expect("");

            let mut items = vec![];
            self.insert_rubies(&item.ruby, |ruby| {
                items.push(RelatedRubyLink {
                    related_id,
                    ruby: ruby.to_owned(),
                })
            });
            diesel::insert_into(crate::schema::related_ruby::dsl::related_ruby::table())
                .values(items)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .expect("");

            let mut items = vec![];
            self.insert_urls(&item.urls, |ur| {
                items.push(RelatedUrlLink {
                    related_id,
                    url: ur.to_owned(),
                })
            });
            diesel::insert_into(crate::schema::related_urls::dsl::related_urls::table())
                .values(items)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .expect("");
        };
    }

    fn insert_categories<F>(&self, categories_dom: Vec<Category>, mut add_link: F)
        where
            F: FnMut(i32),
    {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        for item in categories_dom {
            let new_category = NewCategorie {
                langcode: item.langcode,
                orig: item.orig,
                kind: item.kind,
                name: item.name,
            };

            let inserted: i32 = diesel::insert_into(crate::schema::categories::dsl::categories::table())
                .values(&new_category)
                .on_conflict((crate::schema::categories::name, crate::schema::categories::kind)) // замените `name` и `kind` на соответствующие поля, участвующие в уникальном ограничении
                .do_update()
                .set(&new_category)
                .returning(crate::schema::categories::id)
                .get_result(&mut conn)
                .expect("error while storing categories");

            add_link(inserted)
        }
    }

    fn insert_word_relations(&mut self, item: &dyn Relation, word_id: i32) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        let mut items = vec![];
        self.insert_related(item.derived(), |related_id| {
            items.push(NewWordDerivedLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordderivedlink::dsl::wordderivedlink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.abbreviations(), |related_id| {
            items.push(NewWordAbbreviationsLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordabbreviationslink::dsl::wordabbreviationslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.alt_of(), |related_id| {
            items.push(NewWordAltOfLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordaltoflink::dsl::wordaltoflink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.related(), |related_id| {
            items.push(NewWordRelatedLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordrelatedlink::dsl::wordrelatedlink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.antonyms(), |related_id| {
            items.push(NewWordAntonymsLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordantonymslink::dsl::wordantonymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.coordinate_terms(), |related_id| {
            items.push(NewWordCoordinateTermsLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordcoordinatetermslink::dsl::wordcoordinatetermslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.synonyms(), |related_id| {
            items.push(NewWordSynonymLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordsynonymlink::dsl::wordsynonymlink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.form_of(), |related_id| {
            items.push(NewWordFormOfLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordformoflink::dsl::wordformoflink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.proverbs(), |related_id| {
            items.push(
                NewWordProverbLink {
                    word_id,
                    related_id,
                }
            );
        });
        diesel::insert_into(crate::schema::wordproverbslink::dsl::wordproverbslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.hyponyms(), |related_id| {
            items.push(NewWordHyponymsLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordhyponymslink::dsl::wordhyponymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.hypernyms(), |related_id| {
            items.push(NewWordHypernymsLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordhypernymslink::dsl::wordhypernymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.holonyms(), |related_id| {
            items.push(NewWordHolonymsLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordholonymslink::dsl::wordholonymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.meronyms(), |related_id| {
            items.push(NewWordMeronymsLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordmeronymslink::dsl::wordmeronymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");

        let mut items = vec![];
        self.insert_related(item.troponyms(), |related_id| {
            items.push(NewWordTroponymsLink {
                word_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::wordtroponymslink::dsl::wordtroponymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");
    }

    fn insert_sense_relations(&mut self, item: &dyn Relation, sense_id: i32) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        let mut items = vec![];
        self.insert_related(item.derived(), |related_id| {
            items.push(NewSenseDerivedLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::sensederivedlink::dsl::sensederivedlink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_derived");

        let mut items = vec![];
        self.insert_related(item.abbreviations(), |related_id| {
            items.push(NewSenseAbbreviationsLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::senseabbreviationslink::dsl::senseabbreviationslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_abbreviations");

        let mut items = vec![];
        self.insert_related(item.alt_of(), |related_id| {
            items.push(NewSenseAltOfLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::sensealtoflink::dsl::sensealtoflink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_altof");

        let mut items = vec![];
        self.insert_related(item.related(), |related_id| {
            items.push(NewSenseRelatedLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::senserelatedlink::dsl::senserelatedlink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_related");

        let mut items = vec![];
        self.insert_related(item.antonyms(), |related_id| {
            items.push(NewSenseAntonymsLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::senseantonymslink::dsl::senseantonymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_antonyms");

        let mut items = vec![];
        self.insert_related(item.coordinate_terms(), |related_id| {
            items.push(NewSenseCoordinateTermsLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::sensecoordinatetermslink::dsl::sensecoordinatetermslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_coordinateterms");

        let mut items = vec![];
        self.insert_related(item.synonyms(), |related_id| {
            items.push(NewSenseSynonymLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::sensesynonymlink::dsl::sensesynonymlink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_synonyms");

        let mut items = vec![];
        self.insert_related(item.form_of(), |related_id| {
            items.push(NewSenseFormOfLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::senseformoflink::dsl::senseformoflink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_formof");

        let mut items = vec![];
        self.insert_related(item.proverbs(), |related_id| {
            items.push(
                NewSenseProverbLink {
                    sense_id,
                    related_id,
                }
            );
        });
        diesel::insert_into(crate::schema::senseproverbslink::dsl::senseproverbslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_proverbs");

        let mut items = vec![];
        self.insert_related(item.hyponyms(), |related_id| {
            items.push(NewSenseHyponymsLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::sensehyponymslink::dsl::sensehyponymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_hyponyms");

        let mut items = vec![];
        self.insert_related(item.hypernyms(), |related_id| {
            items.push(NewSenseHypernymsLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::sensehypernymslink::dsl::sensehypernymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_hypernyms");

        let mut items = vec![];
        self.insert_related(item.holonyms(), |related_id| {
            items.push(NewSenseHolonymsLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::senseholonymslink::dsl::senseholonymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_holonyms");

        let mut items = vec![];
        self.insert_related(item.meronyms(), |related_id| {
            items.push(NewSenseMeronymsLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::sensemeronymslink::dsl::sensemeronymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_meronyms");

        let mut items = vec![];
        self.insert_related(item.troponyms(), |related_id| {
            items.push(NewSenseTroponymsLink {
                sense_id,
                related_id,
            });
        });
        diesel::insert_into(crate::schema::sensetroponymslink::dsl::sensetroponymslink::table())
            .values(items)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("insert_sense_troponyms");
    }

    fn insert_tags<F>(&self, tags: &Vec<String>, mut add_link: F)
        where
            F: FnMut(&str) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        let tags: Vec<Tag> = tags.into_iter().map(|tag| {
            add_link(tag);
            Tag { tag: tag.to_owned() }
        }).collect();

        diesel::insert_into(crate::schema::tags::dsl::tags::table())
            .values(tags)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("Something went wrong while inserting tags");
    }

    fn insert_topics<F>(&self, topics: &Vec<String>, mut add_link: F)
        where
            F: FnMut(&str) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        let topics: Vec<Topic> = topics.into_iter().map(|topic| {
            add_link(topic);
            Topic { topic: topic.to_owned() }
        }).collect();
        diesel::insert_into(crate::schema::topics::dsl::topics::table())
            .values(topics)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");
    }

    fn insert_rubies<F>(&self, rubies: &Vec<Vec<String>>, mut add_link: F)
        where
            F: FnMut(&str) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        let rubies: Vec<Ruby> = rubies.into_iter().map(|ruby| {
            let json = serde_json::to_string(&rubies).expect("");
            add_link(&json);
            Ruby {
                ruby: json
            }
        }).collect();

        diesel::insert_into(crate::schema::rubies::dsl::rubies::table())
            .values(rubies)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");
    }

    fn insert_urls<F>(&self, urls: &Vec<String>, mut add_link: F)
        where
            F: FnMut(&str) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        let urls: Vec<Url> = urls.into_iter().map(|ur| {
            add_link(ur);
            Url { url: ur.to_owned() }
        }).collect();

        diesel::insert_into(crate::schema::urls::dsl::urls::table())
            .values(urls)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");
    }

    fn insert_links<F>(&self, links: &Vec<String>, mut add_link: F)
        where
            F: FnMut(&str) {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        let links: Vec<Link> = links.into_iter().map(|link| {
            add_link(link);
            Link { link: link.to_owned() }
        }).collect();

        diesel::insert_into(crate::schema::links::dsl::links::table())
            .values(links)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .expect("");
    }
}

#[async_trait::async_trait]
impl DictionaryRepository for DieselDictionaryRepository {
    async fn bulk_insert(&mut self, items: Vec<crate::domain::models::Word>) -> Result<(), Error> {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        for item in items {
            let new_word = NewWord {
                word: item.word.clone(),
                pos: item.pos.clone(),
                lang_code: item.lang_code.clone(),
                lang: item.lang.clone(),
                etymology_number: item.etymology_number.clone(),
                etymology_text: item.etymology_text.clone(),
            };

            let word_id = diesel::insert_into(crate::schema::words::dsl::words::table())
                .values(new_word)
                .returning(crate::schema::words::id)
                .get_result(&mut conn)
                .expect("");

            self.insert_word_relations(&item, word_id);

            for f in item.forms {
                let new_form = NewForm {
                    word_id,
                    form: f.form,
                    head_nr: f.head_nr,
                    roman: f.roman,
                    ipa: f.ipa,
                };

                let form_id: i32 = diesel::insert_into(crate::schema::forms::dsl::forms::table())
                    .values(new_form)
                    .returning(crate::schema::forms::id)
                    .get_result(&mut conn)
                    .expect("");

                let mut items = vec![];
                self.insert_tags(&f.tags, |tag| {
                    items.push(FormTagLink {
                        forms_id: form_id,
                        tag: tag.to_owned(),
                    })
                });
                diesel::insert_into(crate::schema::forms_tags::dsl::forms_tags::table())
                    .values(items)
                    .execute(&mut conn)
                    .expect("");

                let mut items = vec![];
                self.insert_rubies(&f.ruby, |ruby| {
                    items.push(FormRubyLink {
                        forms_id: form_id,
                        ruby: ruby.to_owned(),
                    })
                });
                diesel::insert_into(crate::schema::forms_ruby::dsl::forms_ruby::table())
                    .values(items)
                    .execute(&mut conn)
                    .expect("");
            }
            let templates: Vec<NewTemplate> = item.etymology_templates.into_iter().map(|template| {
                NewTemplate {
                    word_id,
                    args: serde_json::to_string(&template.args).unwrap(),
                    name: template.name,
                    expansion: template.expansion,
                }
            }).collect();

            diesel::insert_into(crate::schema::templates::dsl::templates::table())
                .values(templates)
                .execute(&mut conn)
                .expect("");

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
                let sound_id: i32 = diesel::insert_into(crate::schema::sounds::dsl::sounds::table())
                    .values(new_sound)
                    .returning(crate::schema::sounds::id)
                    .get_result(&mut conn)
                    .expect("");

                let mut tags = vec![];
                self.insert_tags(&sound.tags, |tag| {
                    tags.push(SoundTagLink { sound_id, tag: tag.to_owned() });
                });

                let mut topics = vec![];
                self.insert_topics(&sound.topics, |topic| {
                    topics.push(SoundTopicLink { sound_id, topic: topic.to_owned() });
                });

                diesel::insert_into(crate::schema::sound_tags::dsl::sound_tags::table())
                    .values(tags)
                    .on_conflict_do_nothing()
                    .execute(&mut conn)
                    .expect("");

                diesel::insert_into(crate::schema::sound_topics::dsl::sound_topics::table())
                    .values(topics)
                    .on_conflict_do_nothing()
                    .execute(&mut conn)
                    .expect("");
            }

            let hyphenations: Vec<NewHyphenation> = item.hyphenation.into_iter().map(|hyphenation| {
                NewHyphenation {
                    word_id,
                    hyphenation,
                }
            }).collect();
            diesel::insert_into(crate::schema::hyphenations::dsl::hyphenations::table())
                .values(hyphenations)
                .execute(&mut conn)
                .expect("");

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

                let sense_id = diesel::insert_into(crate::schema::senses::dsl::senses::table())
                    .values(new_sense)
                    .returning(crate::schema::senses::id)
                    .get_result(&mut conn)
                    .expect("");

                self.insert_sense_relations(&sense, sense_id);

                // TODO: как сделать сохранение информации следующего типа: [[
                //   {
                //     "links": "[\"computing\",\"computing#Noun\"]"
                //   },
                //   {
                //     "links": "[\"copyright\",\"copyright\"]"
                //   }
                // ]]
                // let mut items = vec![];
                // self.insert_links(&sense.links, |link| {
                //     items.push(SenseLinksLink {
                //         sense_id,
                //         link: link.to_owned(),
                //     })
                // });
                // diesel::insert_into(crate::schema::sense_links::dsl::sense_links::table())
                //     .values(items)
                //     .execute(&mut conn)
                //     .expect("");

                let mut items = vec![];
                self.insert_tags(&sense.tags, |tag| {
                    items.push(SenseTagLink {
                        sense_id,
                        tag: tag.to_owned(),
                    })
                });
                diesel::insert_into(crate::schema::sense_tags::dsl::sense_tags::table())
                    .values(items)
                    .execute(&mut conn)
                    .expect("");

                let mut items = vec![];
                self.insert_topics(&sense.topics, |topic| {
                    items.push(SenseTopicLink {
                        sense_id,
                        topic: topic.to_owned(),
                    })
                });
                diesel::insert_into(crate::schema::sense_topics::dsl::sense_topics::table())
                    .values(items)
                    .execute(&mut conn)
                    .expect("");

                self.insert_categories(sense.categories, |category_id| {
                    diesel::insert_into(crate::schema::sensecategorieslink::dsl::sensecategorieslink::table())
                        .values(NewSenseCategoriesLink {
                            sense_id,
                            category_id,
                        })
                        .on_conflict_do_nothing()
                        .execute(&mut conn)
                        .expect("");
                });

                let examples: Vec<NewExample> = sense.examples.into_iter().map(|example| {
                    NewExample {
                        sense_id,
                        note: example.note,
                        example_ref: example.example_ref,
                        roman: example.roman,
                        english: example.english,
                        text: example.text,
                        example_type: example.example_type,
                    }
                }).collect();

                diesel::insert_into(crate::schema::examples::dsl::examples::table())
                    .values(examples)
                    .execute(&mut conn)
                    .expect("");
            }
        }
        Ok(())
    }

    async fn find_exactly(&self, query: &str) -> Result<Option<Vec<crate::domain::models::Word>>, Error> {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        let words_db = crate::schema::words::dsl::words::table()
            .filter(word.eq(query))
            .load::<Word>(&mut conn)
            .expect("Error loading words");

        let results = Self::map_words(&self.pool, words_db);
        return Ok(Some(results));
    }

    async fn find(&self, query: &str) -> Result<Option<Vec<crate::domain::models::Word>>, Error> {
        let pattern = format!("%{}%", query);
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();

        let words_db = crate::schema::words::dsl::words::table()
            .filter(word.ilike(pattern))
            .load::<Word>(&mut conn)
            .expect("Error loading words");

        let results = Self::map_words(&self.pool, words_db);
        return Ok(Some(results));
    }

    async fn random_word(&self, max_symbols: u32) -> crate::domain::models::Word {
        let mut conn: PooledConnection<ConnectionManager<PgConnection>> = self.pool.get().unwrap();
        let count: i64 = 1259153;
        let offset = rand::random::<u64>() % count as u64;

        let mut word_db: Word = crate::schema::words::dsl::words.offset(offset as i64).first(&mut conn).unwrap();

        while word_db.word.len() < 5 || word_db.word.len() > max_symbols as usize {
            word_db = crate::schema::words::dsl::words.offset(offset as i64).first(&mut conn).unwrap();
        }

        return Self::collect_word(&self.pool, word_db);
    }
}
