#![feature(decl_macro, proc_macro_hygiene)]

use juniper::{object, FieldResult, RootNode};

use rocket::{response::content, State};

pub struct Context {
    pub pool: r2d2::Pool<r2d2_diesel::ConnectionManager<PgConnection>>,
}

impl juniper::Context for Context {}

impl Context {
    fn db_connection<F, T>(&self, action: F) -> T
    where
        F: Fn(r2d2::PooledConnection<ConnectionManager<PgConnection>>) -> T,
    {
        let connection = self.pool.get().unwrap();

        action(connection)
    }
}

extern crate diesel;
extern crate diesel_demo;
extern crate r2d2;
extern crate r2d2_diesel;

use self::diesel::prelude::*;
use self::diesel_demo::*;

use diesel_demo::models::*;
use diesel_demo::schema::posts::dsl::*;

pub struct Query {}

#[object(
    Context = Context
)]
impl Query {
    fn all_posts(ctx: &Context) -> FieldResult<Vec<Post>> {
        ctx.db_connection(|db| {
            let all_posts = posts.load::<Post>(&*db).expect("asd");

            Ok(all_posts)
        })
    }
}

pub struct Mutation {}

#[object(
    Context = Context
)]
impl Mutation {
    fn createPost(ctx: &Context, post: NewPost) -> FieldResult<Post> {
        ctx.db_connection(|db| {
            let new_post = diesel::insert_into(schema::posts::table)
                .values(&post)
                .get_result(&*db)
                .expect("error saving");

            Ok(new_post)
        })
    }
}

type Schema = RootNode<'static, Query, Mutation>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    ctx: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &ctx)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    ctx: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &ctx)
}

use std::env;
extern crate dotenv;
use dotenv::dotenv;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let server = rocket::ignite()
        .manage(Context { pool })
        .manage(Schema::new(Query {}, Mutation {}))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler,],
        );

    println!("{}", server.config().port);
    server.launch();
}
