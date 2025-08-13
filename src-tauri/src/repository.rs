use crate::models::Upload;
use crate::GenericResult;
use mikupush_entity::upload;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct UploadRepository {
    db: DatabaseConnection,
}

impl UploadRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> GenericResult<Vec<Upload>> {
        let entities = upload::Entity::find().all(&self.db).await?;

        let models: Vec<Upload> = entities
            .iter()
            .map(|entity| entity.clone().into())
            .collect();

        Ok(models)
    }

    pub async fn find_by_id(&self, id: Uuid) -> GenericResult<Option<Upload>> {
        let entity = upload::Entity::find_by_id(id).one(&self.db).await?;
        Ok(entity.map(|entity| entity.clone().into()))
    }

    pub async fn save(&self, upload: Upload) -> GenericResult<()> {
        let model: upload::Model = upload.clone().into();
        let existing = upload::Entity::find_by_id(upload.id).one(&self.db).await?;
        let active_model: upload::ActiveModel = model.into();

        if existing.is_none() {
            active_model.insert(&self.db).await?;
        } else {
            active_model.update(&self.db).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::database::setup_test_database_connection;
    use crate::models::Upload;
    use crate::repository::UploadRepository;
    use mikupush_entity::upload;
    use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
    use uuid::Uuid;

    #[tokio::test]
    async fn upload_repository_find_all_should_return_all() {
        let db = setup_test_database_connection().await;
        let repository = UploadRepository::new(db.clone());
        let expected: Vec<Upload> = insert_many_test_uploads(&db, 10).await;

        let actual: Vec<Upload> = repository.find_all().await.unwrap();

        assert_eq!(expected.len(), actual.len());
    }

    #[tokio::test]
    async fn upload_repository_find_by_id_should_find_existing() {
        let db = setup_test_database_connection().await;
        let repository = UploadRepository::new(db.clone());
        let expected: Upload = insert_test_upload(&db).await;

        let actual: Option<Upload> = repository.find_by_id(expected.id).await.unwrap();

        assert_eq!(true, actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[tokio::test]
    async fn upload_repository_find_by_id_should_not_find_not_existing() {
        let db = setup_test_database_connection().await;
        let repository = UploadRepository::new(db.clone());

        let actual: Option<Upload> = repository.find_by_id(Uuid::new_v4()).await.unwrap();

        assert_eq!(true, actual.is_none());
    }

    #[tokio::test]
    async fn upload_repository_save_should_insert_new_model() {
        let db = setup_test_database_connection().await;
        let repository = UploadRepository::new(db.clone());
        let upload = Upload::test();

        repository.save(upload.clone()).await.unwrap();
        let existing = upload::Entity::find_by_id(upload.id)
            .one(&db)
            .await
            .unwrap();

        assert_eq!(true, existing.is_some());
        assert_eq!(upload, existing.unwrap().into());
    }

    async fn insert_test_upload(db: &DatabaseConnection) -> Upload {
        let model: upload::Model = Upload::test().into();
        let active_model: upload::ActiveModel = model.clone().into();
        let result = active_model.insert(db).await;
        println!("insert test upload result: {:?}", result);

        result.unwrap().into()
    }

    async fn insert_many_test_uploads(db: &DatabaseConnection, count: i8) -> Vec<Upload> {
        let mut uploads: Vec<Upload> = vec![];

        for _ in 0..count {
            uploads.push(insert_test_upload(db).await);
        }

        uploads
    }
}
