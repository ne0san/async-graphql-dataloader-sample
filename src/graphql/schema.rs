use super::resolver::{item::ItemLoaderByUser, user::UserLoader};
use crate::graphql::resolver::Query;
use async_graphql::{dataloader::DataLoader, EmptyMutation, EmptySubscription, Schema};
use sea_orm::DatabaseConnection;
use tokio::task;

pub type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub async fn build_schema(db: DatabaseConnection) -> ApiSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db.clone())
        .data(DataLoader::new(UserLoader::new(db.clone()), task::spawn))
        .data(DataLoader::new(ItemLoaderByUser::new(db), task::spawn))
        .limit_recursive_depth(5)
        .finish()
}
