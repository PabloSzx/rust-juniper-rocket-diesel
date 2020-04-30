use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Queryable, GraphQLObject)]
#[graphql(description = "Post XD")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::posts;

#[graphql(description = "New Post XD")]
#[derive(Insertable, GraphQLInputObject)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
}
