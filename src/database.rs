use crate::shape_service::Shape;
use async_graphql::Object;
use async_graphql::ID;
pub struct Database;

impl Database {
    pub fn get_data(&self) -> Vec<Shape> {
        vec![
            Shape {
                id: ID::from("1"),
                name: String::from("Square 1"),
            },
            Shape {
                id: ID::from("2"),
                name: String::from("Triangle 1"),
            },
        ]
    }
}

#[Object]
impl Shape {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
}
