use super::{item::Item, user::User};
use ::entity::{item, user};
use async_graphql::{Context, Error as GraphQLError, Object, Result as GraphQLResult, ID};
use sea_orm::*;

pub struct Query;
#[Object]
impl Query {
    async fn user(&self, ctx: &Context<'_>, id: Option<ID>) -> GraphQLResult<Vec<User>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(id) = id {
            let id: i32 = id.parse()?;
            let data = user::Entity::find_by_id(id).one(db).await;
            if let Ok(Some(data)) = data {
                Ok(vec![User::new(data)])
            } else {
                Err(GraphQLError::new("Fail to find user"))
            }
        } else {
            let data = user::Entity::find().all(db).await;
            if let Ok(data) = data {
                Ok(data.into_iter().map(|d| User::new(d)).collect())
            } else {
                Err(GraphQLError::new("Fail to find users"))
            }
        }
    }

    async fn item(&self, ctx: &Context<'_>, id: Option<ID>) -> GraphQLResult<Vec<Item>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        if let Some(id) = id {
            let id: i32 = id.parse()?;
            let data = item::Entity::find_by_id(id).one(db).await;
            if let Ok(Some(data)) = data {
                Ok(vec![Item::new(data)])
            } else {
                Err(GraphQLError::new("Fail to find item"))
            }
        } else {
            let data = item::Entity::find().all(db).await;
            if let Ok(data) = data {
                Ok(data.into_iter().map(|d| Item::new(d)).collect())
            } else {
                Err(GraphQLError::new("Fail to find items"))
            }
        }
    }
}
