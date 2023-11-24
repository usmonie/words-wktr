mod models;


use diesel::associations::{BelongsTo, HasTable};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use tokio::{join, task};
use crate::data::models::{NewArg, NewCategory, NewCategoryParent, NewDerivedWord, NewEtymologyTemplate, NewForm, NewGloss, NewHyphenation, NewLink, NewRawGloss, NewSense, NewSenseTag, NewSound, NewSoundTag, NewTag, NewWord};
use crate::domain::Error;
use crate::domain::models::{Category, DerivedWord, EtymologyTemplate, Form, Sense, Word};
use crate::domain::use_cases::DictionaryRepository;
use crate::schema::category_parents::dsl::category_parents;
use crate::schema::derived_words::dsl::derived_words;
use crate::schema::forms::dsl::forms;
use crate::schema::glosss::dsl::glosss;
use crate::schema::links::dsl::links;
use crate::schema::raw_glosss::dsl::raw_glosss;
use crate::schema::sense_tags::dsl::sense_tags;
use crate::schema::args::dsl::args;
use crate::schema::categorys::dsl::categorys;
use crate::schema::etymology_templates::dsl::etymology_templates;
use crate::schema::hyphenations::dsl::hyphenations;
use crate::schema::senses::dsl::senses;
use crate::schema::sound_tags::dsl::sound_tags;
use crate::schema::sounds::dsl::sounds;
use crate::schema::tags::dsl::tags;

pub struct DieselDictionaryRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselDictionaryRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to crate database pool");

        Self { pool }
    }

    fn get_forms(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_word: &models::Word) -> Vec<Form> {
        let forms_mapped = models::Form::belonging_to(&other_word)
            .load::<models::Form>(conn)
            .expect("");

        let forms_mapped = forms_mapped.into_iter()
            .map(|form| {
                let other_tags = models::Tag::belonging_to(&form)
                    .load::<models::Tag>(conn)
                    .expect("")
                    .into_iter()
                    .map(|tag| tag.tag)
                    .collect::<Vec<_>>();

                Form {
                    form: form.form,
                    tags: other_tags,
                }
            }).collect::<Vec<_>>();
        forms_mapped
    }

    fn get_templates(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_word: &models::Word) -> Vec<EtymologyTemplate> {
        let templates_mapped = models::EtymologyTemplate::belonging_to(&other_word)
            .load::<models::EtymologyTemplate>(conn)
            .expect("")
            .into_iter()
            .map(|other_template| {
                let args_mapped = models::Arg::belonging_to(&other_template)
                    .load::<models::Arg>(conn)
                    .expect("")
                    .into_iter()
                    .map(|other_arg| { (other_arg.arg_key, other_arg.arg_val) })
                    .collect();

                EtymologyTemplate {
                    name: other_template.name,
                    expansion: other_template.expansion,
                    args: args_mapped,
                }
            }).collect::<Vec<_>>();
        templates_mapped
    }

    fn get_sounds(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_word: &models::Word) -> Vec<crate::domain::models::Sound> {
        let sounds_mapped = models::Sound::belonging_to(&other_word)
            .load::<models::Sound>(conn)
            .expect("")
            .into_iter()
            .map(|other_sound| {
                let sound_tags_mapped = models::SoundTag::belonging_to(&other_sound)
                    .load::<models::SoundTag>(conn)
                    .expect("")
                    .into_iter()
                    .map(|other_tag| { other_tag.tag })
                    .collect();

                crate::domain::models::Sound {
                    ipa: other_sound.ipa,
                    enpr: other_sound.enpr,
                    audio: other_sound.audio,
                    text: other_sound.text,
                    tags: Some(sound_tags_mapped),
                    ogg_url: other_sound.ogg_url,
                    mp3_url: other_sound.mp3_url,
                    homophone: other_sound.homophone,
                    rhymes: other_sound.rhymes,
                }
            }).collect::<Vec<_>>();
        sounds_mapped
    }

    fn get_hyphenations(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_word: &models::Word) -> Vec<String> {
        let hyphenations_mapped = models::Hyphenation::belonging_to(&other_word)
            .load::<models::Hyphenation>(conn)
            .expect("")
            .into_iter()
            .map(|other_hyphenation| {
                other_hyphenation.hyphen
            }).collect::<Vec<_>>();

        hyphenations_mapped
    }

    fn get_derived(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_word: &models::Word) -> Vec<DerivedWord> {
        let derived = models::DerivedWord::belonging_to(&other_word)
            .load::<models::DerivedWord>(conn)
            .expect("")
            .into_iter()
            .map(|other_derived| {
                DerivedWord {
                    word: other_derived.word,
                    _dis1: other_derived._dis1,
                }
            }).collect::<Vec<_>>();
        derived
    }

    async fn get_senses(pool: &Pool<ConnectionManager<PgConnection>>, other_word: &models::Word) -> Vec<crate::domain::models::Sense> {
        let mut conn = pool.get().unwrap();
        let mut senses_mapped = vec![];
        let senses_loaded = models::Sense::belonging_to(&other_word)
            .load::<models::Sense>(&mut conn)
            .expect("");

        for other_sense in senses_loaded {
            let s_c = other_sense.clone();
            let mut pool_c = pool.clone();
            let links_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_links(&mut conn, &s_c)
            });
            let s_c = other_sense.clone();
            let mut pool_c = pool.clone();

            let raw_glosses_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_raw_glosses(&mut conn, &s_c)
            });
            let s_c = other_sense.clone();
            let mut pool_c = pool.clone();

            let glosses_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_glosses(&mut conn, &s_c)
            });
            let s_c = other_sense.clone();
            let mut pool_c = pool.clone();

            let tags_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_tags(&mut conn, &s_c)
            });
            let s_c = other_sense.clone();
            let mut pool_c = pool.clone();

            let categories_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_categories(&mut conn, &s_c)
            });

            let (
                links_mapped,
                raw_glosses_mapped,
                glosses_mapped,
                tags_mapped,
                categories_mapped,
            ) = join!(links_task.await.unwrap(), raw_glosses_task.await.unwrap(), glosses_task.await.unwrap(), tags_task.await.unwrap(), categories_task.await.unwrap());

            senses_mapped.push(
                Sense {
                    links: Some(links_mapped),
                    raw_glosses: Some(raw_glosses_mapped),
                    glosses: Some(glosses_mapped),
                    tags: Some(tags_mapped),
                    id: other_sense.id_1,
                    categories: Some(categories_mapped),
                }
            );
        }
        senses_mapped
    }

    fn get_categories(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_sense: &models::Sense) -> Vec<Category> {
        let categories_mapped = models::Category::belonging_to(&other_sense)
            .load::<models::Category>(conn)
            .expect("Failed to load categories")
            .into_iter()
            .map(|category| {
                let parent_mapped = models::CategoryParent::belonging_to(&category)
                    .load::<models::CategoryParent>(conn)
                    .expect("Failed to load categories")
                    .into_iter()
                    .map(|category_parent| {
                        category_parent.parent
                    })
                    .collect();

                crate::domain::models::Category {
                    name: category.name,
                    kind: category.kind,
                    source: category.source,
                    orig: category.orig,
                    langcode: category.langcode,
                    _dis: category._dis,
                    parents: Some(parent_mapped),
                }
            })
            .collect::<Vec<_>>();
        categories_mapped
    }

    fn get_tags(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_sense: &models::Sense) -> Vec<String> {
        let tags_mapped = models::SenseTag::belonging_to(&other_sense)
            .load::<models::SenseTag>(conn)
            .expect("Failed to load sense_tags")
            .into_iter()
            .map(|sense_tag| { sense_tag.tag })
            .collect::<Vec<_>>();
        tags_mapped
    }

    fn get_glosses(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_sense: &models::Sense) -> Vec<String> {
        let glosses_mapped = models::Gloss::belonging_to(&other_sense)
            .load::<models::Gloss>(conn)
            .expect("Failed to load glosses")
            .into_iter()
            .map(|gloss| { gloss.gloss })
            .collect::<Vec<_>>();
        glosses_mapped
    }

    fn get_raw_glosses(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_sense: &models::Sense) -> Vec<String> {
        let raw_glosses_mapped = models::RawGloss::belonging_to(&other_sense)
            .load::<models::RawGloss>(conn)
            .expect("Failed to load raw_glosses")
            .into_iter()
            .map(|raw_gloss| { raw_gloss.raw_gloss })
            .collect::<Vec<_>>();
        raw_glosses_mapped
    }

    fn get_links(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, other_sense: &models::Sense) -> Vec<Vec<String>> {
        let links_mapped = models::Link::belonging_to(&other_sense)
            .load::<models::Link>(conn)
            .expect("Failed to load links")
            .into_iter()
            .map(|link| { vec![link.link_1, link.link_2] })
            .collect::<Vec<_>>();
        links_mapped
    }
}

#[async_trait::async_trait]
impl DictionaryRepository for DieselDictionaryRepository {
    fn bulk_insert(&mut self, items: Vec<Word>) -> Result<(), Error> {
        use crate::schema::words::dsl::*;

        let mut conn = self.pool.get().unwrap();

        for item in items {
            let new_word = NewWord {
                word: item.word,
                pos: item.pos,
                lang_code: item.lang_code,
                lang: item.lang,
                etymology_number: item.etymology_number,
                etymology_text: item.etymology_text,
            };

            let word_id = diesel::insert_into(words::table())
                .values(new_word)
                .returning(id)
                .get_result(&mut conn)
                .expect("");

            for f in item.forms {
                let new_form = NewForm { word_id, form: f.form };
                let form_id = diesel::insert_into(forms::table())
                    .values(new_form)
                    .returning(crate::schema::forms::id)
                    .get_result(&mut conn)
                    .expect("");


                for tag in f.tags {
                    let new_tag = NewTag { form_id, tag: Some(tag) };
                    diesel::insert_into(tags::table())
                        .values(new_tag)
                        .execute(&mut conn)
                        .expect("");
                }
            }

            for template in item.etymology_templates {
                let new_template = NewEtymologyTemplate {
                    word_id,
                    name: template.name,
                    expansion: template.expansion,
                };
                let template_id: i32 = diesel::insert_into(etymology_templates::table())
                    .values(new_template)
                    .returning(crate::schema::etymology_templates::id)
                    .get_result(&mut conn)
                    .expect("");

                for (key, val) in template.args {
                    let new_arg = NewArg {
                        etymology_template_id: template_id,
                        arg_key: Some(key),
                        arg_val: Some(val),
                    };
                    diesel::insert_into(args::table())
                        .values(new_arg)
                        .execute(&mut conn)
                        .expect("");
                }
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
                };
                let sound_id = diesel::insert_into(sounds::table())
                    .values(new_sound)
                    .returning(crate::schema::sounds::id)
                    .get_result(&mut conn)
                    .expect("");

                if let Some(tags_items) = sound.tags {
                    for tag in tags_items {
                        let new_sound_tag = NewSoundTag {
                            sound_id,
                            tag: Some(tag),
                        };
                        diesel::insert_into(sound_tags::table())
                            .values(new_sound_tag)
                            .execute(&mut conn)
                            .expect("");
                    }
                }
            }

            for hyphenation_item in item.hyphenation {
                let new_hyphen = NewHyphenation {
                    word_id,
                    hyphen: Some(hyphenation_item),
                };
                diesel::insert_into(hyphenations::table())
                    .values(new_hyphen)
                    .execute(&mut conn)
                    .expect("");
            }

            for derived_word in item.derived {
                let new_derived_word = NewDerivedWord {
                    word_id,
                    word: derived_word.word,
                    _dis1: derived_word._dis1,
                };
                diesel::insert_into(derived_words::table())
                    .values(new_derived_word)
                    .execute(&mut conn)
                    .expect("");
            }

            for sense in item.senses {
                let new_sense = NewSense {
                    word_id,
                    id_1: sense.id,
                };
                let sense_id = diesel::insert_into(senses::table())
                    .values(new_sense)
                    .returning(crate::schema::senses::id)
                    .get_result(&mut conn)
                    .expect("");

                if let Some(links_domain) = sense.links {
                    for link in links_domain {
                        let new_link = NewLink {
                            sense_id,
                            link_1: Some(link[0].clone()),
                            link_2: Some(link[1].clone()),
                        };
                        diesel::insert_into(links::table())
                            .values(new_link)
                            .execute(&mut conn)
                            .expect("");
                    }
                }

                if let Some(raw_glosses_domain) = sense.raw_glosses {
                    for raw_gloss in raw_glosses_domain {
                        let new_raw_gloss = NewRawGloss {
                            sense_id,
                            raw_gloss: Some(raw_gloss),
                        };
                        diesel::insert_into(raw_glosss::table())
                            .values(new_raw_gloss)
                            .execute(&mut conn)
                            .expect("");
                    }
                }

                if let Some(glosses_domain) = sense.glosses {
                    for gloss in glosses_domain {
                        let new_gloss = NewGloss {
                            sense_id,
                            gloss: Some(gloss),
                        };
                        diesel::insert_into(glosss::table())
                            .values(new_gloss)
                            .execute(&mut conn)
                            .expect("");
                    }
                }

                if let Some(sense_tags_domain) = sense.tags {
                    for sense_tag in sense_tags_domain {
                        let new_sense_tag = NewSenseTag {
                            sense_id,
                            tag: Some(sense_tag),
                        };
                        diesel::insert_into(sense_tags::table())
                            .values(new_sense_tag)
                            .execute(&mut conn)
                            .expect("");
                    }
                }

                if let Some(categories_domain) = sense.categories {
                    for category in categories_domain {
                        let new_category = NewCategory {
                            sense_id,
                            name: category.name,
                            kind: category.kind,
                            source: category.source,
                            orig: category.orig,
                            langcode: category.langcode,
                            _dis: category._dis,
                        };

                        let category_id = diesel::insert_into(categorys::table())
                            .values(new_category)
                            .returning(crate::schema::categorys::dsl::id)
                            .get_result(&mut conn)
                            .expect("");

                        if let Some(parents_domain) = category.parents {
                            for parent in parents_domain {
                                let new_category_parent = NewCategoryParent {
                                    category_id,
                                    parent: Some(parent),
                                };
                                diesel::insert_into(category_parents::table())
                                    .values(new_category_parent)
                                    .execute(&mut conn)
                                    .expect("");
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn find(&self, query: &str) -> Result<Option<Vec<Word>>, Error> {
        use crate::schema::words::dsl::*;

        let pool = self.pool.clone(); // Clone the connection pool
        let mut conn = pool.get().expect("");

        let found = words
            .filter(word.eq(query))
            .load::<models::Word>(&mut conn)
            .expect("");

        let mut words_m = vec![];

        for w in found {
            let w_c = w.clone();
            let mut pool_c = pool.clone();
            let forms_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_forms(&mut conn, &w_c)
            });
            let w_c = w.clone();
            let mut pool_c = pool.clone();

            let templates_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_templates(&mut conn, &w_c)
            });
            let w_c = w.clone();
            let mut pool_c = pool.clone();

            let sounds_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_sounds(&mut conn, &w_c)
            });
            let w_c = w.clone();
            let mut pool_c = pool.clone();

            let hyphenations_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_hyphenations(&mut conn, &w_c)
            });
            let w_c = w.clone();
            let mut pool_c = pool.clone();

            let derived_task = task::spawn_blocking(move || async move {
                let mut conn = pool_c.get().expect("");
                Self::get_derived(&mut conn, &w_c)
            });

            let w_c = w.clone();
            let mut pool_c = pool.clone();
            let senses_task = task::spawn_blocking(move || async move {
                Self::get_senses(&mut pool_c, &w_c).await
            });

            let (
                forms_m,
                temp_m,
                sounds_m,
                hyphenations_m,
                derived_m,
                sense_m
            ) = join!(
                forms_task.await.unwrap(),
                templates_task.await.unwrap(),
                sounds_task.await.unwrap(),
                hyphenations_task.await.unwrap(),
                derived_task.await.unwrap(),
                senses_task.await.unwrap()
            );

            words_m.push(
                Word {
                    word: w.word.clone(),
                    pos: w.pos.clone(),
                    forms: forms_m,
                    etymology_number: w.etymology_number.clone(),
                    etymology_text: w.etymology_text.clone(),
                    etymology_templates: temp_m,
                    lang: w.lang.clone(),
                    lang_code: w.lang_code.clone(),
                    sounds: sounds_m,
                    hyphenation: hyphenations_m,
                    derived: derived_m,
                    senses: sense_m,
                }
            );
        }

        // match result {
        //     Ok(word) => Ok(if word.is_empty() { None } else { Some(word) }),
        //     Err(error) => Err(error),
        // }

        if words_m.is_empty() {
            Ok(None)
        } else {
            Ok(Some(words_m))
        }
    }
}