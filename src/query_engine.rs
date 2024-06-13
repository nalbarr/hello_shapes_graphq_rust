use crate::database::Database;
use crate::shape_service::Shape;
use async_graphql::Object;

pub struct Query {
    pub database: Database,
}

#[Object]
impl Query {
    async fn get_shape(&self, id: String) -> Option<Shape> {
        self.database
            .get_data()
            .iter()
            .find(|&x| x.id.0 == id)
            .cloned()
    }

    async fn get_shapes(&self) -> Vec<Shape> {
        self.database.get_data()
    }
}
