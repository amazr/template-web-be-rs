use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    DatabaseConnection, EntityTrait,
};
use uuid::Uuid;

use crate::{
    api::users::user::NewUser,
    entities::users::{ActiveModel, Entity, Model},
    errors::{Error, Result},
};

#[derive(Clone)]
pub struct UserStore {
    db: DatabaseConnection,
}

pub type UsersTable = Entity;
pub type NewStoredUser = ActiveModel;
pub type StoredUser = Model;

impl UserStore {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_users(&self) -> Result<Vec<StoredUser>> {
        Ok(UsersTable::find().all(&self.db).await?)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Option<StoredUser>> {
        Ok(UsersTable::find_by_id(id).one(&self.db).await?)
    }

    pub async fn store_user(&self, user: NewUser) -> Result<StoredUser> {
        Ok(NewStoredUser {
            id: Set(Uuid::new_v4()),
            name: Set(user.name),
            email: Set(user.email),
        }
        .insert(&self.db)
        .await?)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        let result = UsersTable::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 1 {
            Ok(())
        } else {
            Err(Error::RowsAffectedUnexpected {
                expected: 1,
                affected: result.rows_affected,
            })
        }
    }
}
