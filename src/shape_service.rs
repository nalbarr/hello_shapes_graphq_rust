use async_graphql::ID;
#[derive(Clone)]
pub struct Shape {
    pub id: ID,
    pub name: String,
}
