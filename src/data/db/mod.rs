pub mod models;
pub mod links;
//
// use diesel::PgConnection;
// use crate::data::db::links::NewWordDerivedLink;
//
// pub trait RelationalEntity {
//     type TableModel; // Основная модель для вставки
//
//     fn map_to_table_model(&self) -> Self::TableModel;
//     fn handle_related_data(&self, conn: &PgConnection);
// }
//
// pub enum Relatable<T: Relation> {
//     Word { word_id: i32 },
//     Sense{ sense_id: i32 },
// }
//
// pub trait Relation {
//     type Model;
//     fn create(related_id: i32, relatable_id: i32) -> Self::Model;
// }
//
// impl Relation for NewWordDerivedLink {
//     type Model = NewWordDerivedLink;
//
//     fn create(related_id: i32, relatable_id: i32) -> Self {
//         NewWordDerivedLink { word_id: relatable_id, related_id }
//     }
// }