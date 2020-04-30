#![feature(decl_macro, proc_macro_hygiene)]

use juniper::{object, FieldResult, RootNode};

use rocket::{response::content, State};

pub struct Database {}

impl juniper::Context for Database {}

extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;

use diesel_demo::models::*;
use diesel_demo::schema::posts::dsl::*;

pub struct Query {}

#[object]
impl Query {
    fn allPosts() -> FieldResult<Vec<Post>> {
        let connection = establish_connection();

        let all_posts = posts.load::<Post>(&connection).expect("asd");

        Ok(all_posts)
    }
}

pub struct Mutation {}

#[object]
impl Mutation {
    fn createPost(post: NewPost) -> FieldResult<Post> {
        let connection = establish_connection();

        let new_post = diesel::insert_into(schema::posts::table)
            .values(post)
            .get_result(&connection)
            .expect("error saving");

        Ok(new_post)
    }
}

type Schema = RootNode<'static, Query, Mutation>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &())
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &())
}

fn main() {
    let server = rocket::ignite()
        .manage(Schema::new(Query {}, Mutation {}))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler,],
        );

    println!("{}", server.config().port);
    server.launch();
}
