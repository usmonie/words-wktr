// use diesel::{PgConnection, RunQueryDsl};
// use diesel::associations::HasTable;
// use diesel::r2d2::{ConnectionManager, Pool};
// use crate::data::db::{Relatable,RelationalEntity};
// use crate::data::db::models::Related;
//
// pub struct DieselDictionaryRepository {
//     pool: Pool<ConnectionManager<PgConnection>>,
// }
// impl DieselDictionaryRepository {
//     fn store_related<L>(&self, relatable: Relatable, items: Vec<Related>, parent_id: i32)
//     {
//         let mut conn = self.pool.get().expect("Failed to get connection");
//
//         for item in items {
//             let insertable = item.new_insertable();
//             let inserted_id: i32 = diesel::insert_into(crate::schema::related::dsl::related::table())
//                 .values(insertable)
//                 .returning(crate::schema::related::id)
//                 .get_result(&mut conn)
//                 .expect("Error while storing relations");
//
//         }
//     }
//
//     fn create_relation(conn: &PgConnection, relatable: Relatable, related_id: i32) -> QueryResult<usize> {
//         match relatable {
//             Relatable::Word(word_id) => {
//                 let new_relation = NewWordsRelatedLink { word_id, related_id };
//                 diesel::insert_into(words_related::table)
//                     .values(new_relation)
//                     .execute(conn)
//             },
//             Relatable::Sense(sense_id) => {
//                 let new_relation = NewSensesRelated { sense_id, related_id };
//                 diesel::insert_into(senses_related::table)
//                     .values(new_relation)
//                     .execute(conn)
//             },
//         }
//     }
// }