use super::item::{Item, ItemLoaderByUser};
use ::entity::{item, user};
use async_graphql::{
    dataloader::{DataLoader, Loader},
    Context, FieldError, Object, Result as GraphQLResult, ID,
};
use sea_orm::entity::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct User(user::Model);

impl User {
    pub fn new(model: user::Model) -> Self {
        Self(model)
    }
}

#[Object]
impl User {
    async fn id(&self) -> ID {
        ID::from(self.0.id)
    }
    async fn name(&self) -> String {
        self.0.name.to_string()
    }
    async fn description(&self) -> Option<String> {
        if let Some(desc) = &self.0.description {
            Some(desc.to_string())
        } else {
            None
        }
    }
    async fn items(&self, ctx: &Context<'_>) -> GraphQLResult<Option<Vec<Item>>> {
        let loader = ctx.data::<DataLoader<ItemLoaderByUser>>().unwrap();
        loader.load_one(self.0.id).await
    }
    async fn base_items(&self, ctx: &Context<'_>) -> Option<Vec<Item>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let data = self.0.find_related(item::Entity).all(db).await;
        if let Ok(data) = data {
            Some(data.into_iter().map(|d| Item::new(d)).collect())
        } else {
            None
        }
    }
}

pub struct UserLoader(DatabaseConnection);
impl UserLoader {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

impl Loader<i32> for UserLoader {
    type Value = User;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        println!("load user by userId {:?}", keys);

        let data = user::Entity::find()
            .filter(user::Column::Id.is_in(keys.iter().copied()))
            .all(&self.0)
            .await?;
        Ok(data.into_iter().map(|d| (d.id, User(d))).collect())
    }
}
