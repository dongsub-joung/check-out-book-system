use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::schema::*;
use crate::models::*;

pub struct BookRepository;

impl BookRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32)-> QueryResult<Book>{
        // @TODO book SCHEMA
        book::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64)-> QueryResult<Vec<Book>>{
        rustaceans::table.limit(limit).get_results(c).await
    }

    // NewRustacean?
    pub async fn create(c: &mut AsyncPgConnection, new_book: NewRustacean) -> QueryResult<Rustacean>{
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, rustacean: Rustacean) -> QueryResult<Rustacean>{
        diesel::update(rustaceans::table.find(rustacean.id))
            .set((
                rustaceans::name.eq(rustacean.name), // so hard
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(rustaceans::table.find(id)).execute(c).await
    }
}