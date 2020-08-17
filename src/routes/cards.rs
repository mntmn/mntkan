use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::*;
use crate::schema::cards::dsl::*;
use diesel::dsl::*;

use actix_web::{error, get, put, post, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use uuid::Uuid;
use chrono::prelude::*;

use crate::models::Card;

pub fn utc_iso_date_string(utc: &DateTime<Utc>) -> String {
    format!("{:04}-{:02}-{:02}", utc.year(), utc.month(), utc.day())
}

#[derive(serde::Deserialize)]
pub struct Query {
    limit: Option<i64>
}

pub fn get_all_cards(conn: &SqliteConnection, q: &Query) -> Vec<Card> {
    // reason for into_boxed: https://github.com/diesel-rs/diesel/issues/455
    let s = cards.into_boxed();
    
    let s = match q.limit {
        Some(limit) => s.limit(limit),
        _ => s.limit(1000)
    };

    s.load::<Card>(conn).unwrap()
}

pub fn get_card_by_id(conn: &SqliteConnection, uuid: &String) -> Result<Card, diesel::result::Error> {
    cards.find(uuid).first(conn)
}

#[derive(serde::Deserialize)]
pub struct NewCard {
    list_id: String,
    title: String,
    body: String
}

pub fn insert_card(conn: &SqliteConnection, nc: &NewCard) -> Result<Card, diesel::result::Error> {
    let card = Card {
        id: Uuid::new_v4().to_string(),
        list_id: nc.list_id.clone(),
        title: nc.title.clone(),
        body: nc.body.clone(),
        updated_at: utc_iso_date_string(&Utc::now()), // FIXME missing time?
        created_at: utc_iso_date_string(&Utc::now()),
    };

    let res = diesel::insert_into(cards).values(&card).execute(conn);
    println!("insert_card result: {:?}", res);
    Ok(card)
}

pub fn update_card(conn: &SqliteConnection, card_id: &String, nc: &NewCard) -> Result<usize, diesel::result::Error> {
    let res = diesel::update(cards.filter(id.eq(card_id)))
        .set((list_id.eq(nc.list_id.clone()),
              title.eq(nc.title.clone()),
              body.eq(nc.body.clone()),
              updated_at.eq(utc_iso_date_string(&Utc::now())),
              ))
        .execute(conn);

    println!("update_card result: {:?}", res);
    return res
}

#[get("/cards")]
pub async fn get_cards(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    
    let s = tmpl.render("cards.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/cards.json")]
pub async fn get_cards_json(
    pool: web::Data<DbPool>,
    q: web::Query<Query>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results:Vec<Card> = get_all_cards(&conn, &q);
    Ok(HttpResponse::Ok().json(results))
}

#[post("/cards.json")]
pub async fn post_card_json(
    pool: web::Data<DbPool>,
    params: web::Json<NewCard>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    match insert_card(&conn, &params) {
        Ok(card) => Ok(HttpResponse::Ok().json(&card)),
        Err(e) => Err(error::ErrorBadRequest(format!("{:?}", e)))
    }
}

#[put("/cards/{id}.json")]
pub async fn put_card_json(
    pool: web::Data<DbPool>,
    params: web::Json<NewCard>,
    path: web::Path<(String,)>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    match update_card(&conn, &path.0, &params) {
        Ok(0) => Err(error::ErrorBadRequest("Update failed.".to_string())),
        Err(e) => Err(error::ErrorBadRequest(format!("Update failed: {:?}.",e))),
        _ => {
            match get_card_by_id(&conn, &path.0) {
                Ok(card) => Ok(HttpResponse::Ok().json(&card)),
                Err(e) => Err(error::ErrorBadRequest(format!("{:?}", e)))
            }
        }
    }
}
