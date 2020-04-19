#[macro_use]
extern crate diesel;


use actix_web::{App, Error, get, HttpRequest, HttpResponse, HttpServer, post, web};
use actix_web_actors::ws;
use diesel::prelude::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::run_pending_migrations;
use listenfd::ListenFd;
use uuid::Uuid;

use crate::actors::MyWs;
use crate::database::{actions, models};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod database;
mod actors;

#[get("/ws")]
async fn indexws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("index"))
}

#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    println!("Get user");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_uid(user_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

#[get("/users")]
async fn get_users(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    println!("Get users");
    let conn = pool.get().expect("couldn't get db connection from pool");
    let users = web::block(move || actions::get_all_users(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if !users.is_empty() {
        Ok(HttpResponse::Ok().json(users))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No users found"));
        Ok(res)
    }
}

#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::insert_new_user(&form.name, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Start");
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let manager = ConnectionManager::<PgConnection>::new(connspec);


    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");


    let conn = pool.get().expect("couldn't get db connection from pool");
    run_pending_migrations(&conn).unwrap();


    let mut server = HttpServer::new(move ||
        App::new()
            .data(pool.clone())
            .service(index)
            .service(get_user)
            .service(add_user)
            .service(get_users)
            .service(indexws)
    );

    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("0.0.0.0:8000")?
    };

    server.run().await
}