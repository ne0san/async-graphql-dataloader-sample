use super::user::{User, UserLoader};
use ::entity::{item, user};
use async_graphql::{
    dataloader::{DataLoader, Loader},
    Context, FieldError, Object, Result as GraphQLResult, ID,
};
use sea_orm::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Item(item::Model);

impl Item {
    pub fn new(model: item::Model) -> Self {
        Self(model)
    }
}

#[Object]
impl Item {
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
    async fn user(&self, ctx: &Context<'_>) -> GraphQLResult<Option<User>> {
        let loader = ctx.data::<DataLoader<UserLoader>>().unwrap();
        loader.load_one(self.0.user_id).await
    }
    async fn base_user(&self, ctx: &Context<'_>) -> Option<User> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let data = self.0.find_related(user::Entity).one(db).await;
        if let Ok(Some(data)) = data {
            Some(User::new(data))
        } else {
            None
        }
    }
}

pub struct ItemLoaderByUser(DatabaseConnection);
impl ItemLoaderByUser {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }
}

impl Loader<i32> for ItemLoaderByUser {
    type Value = Vec<Item>;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        println!("load item by userId {:?}", keys);

        let data = item::Entity::find()
            .filter(item::Column::UserId.is_in(keys.iter().copied()))
            .all(&self.0)
            .await?;
        Ok(data.into_iter().fold(HashMap::new(), |mut acc, d| {
            acc.entry(d.user_id).or_insert_with(Vec::new).push(Item(d));
            acc
        }))
    }
}
