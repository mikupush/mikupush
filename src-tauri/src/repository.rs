use crate::models::Upload;
use crate::GenericResult;
use mikupush_entity::upload;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct UploadRepository {
    db: DatabaseConnection
}

impl UploadRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> GenericResult<Vec<Upload>> {
        let entities = upload::Entity::find().all(&self.db).await?;

        let models: Vec<Upload> = entities.iter()
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
    use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
    use uuid::Uuid;
    use mikupush_entity::upload;
    use crate::database::setup_test_database_connection;
    use crate::models::Upload;
    use crate::repository::UploadRepository;

    #[test]
    fn upload_repository_find_all_should_return_all() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let db = runtime.block_on(setup_test_database_connection());
        let repository = UploadRepository::new(db.clone());
        let expected: Vec<Upload> = runtime.block_on(insert_many_test_uploads(&db, 10));

        let actual: Vec<Upload> = runtime.block_on(repository.find_all()).unwrap();

        assert_eq!(expected.len(), actual.len());
    }

    #[test]
    fn upload_repository_find_by_id_should_find_existing() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let db = runtime.block_on(setup_test_database_connection());
        let repository = UploadRepository::new(db.clone());
        let expected: Upload = runtime.block_on(insert_test_upload(&db));

        let actual: Option<Upload> = runtime.block_on(repository.find_by_id(expected.id)).unwrap();

        assert_eq!(true, actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn upload_repository_find_by_id_should_not_find_not_existing() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let db = runtime.block_on(setup_test_database_connection());
        let repository = UploadRepository::new(db.clone());

        let actual: Option<Upload> = runtime.block_on(repository.find_by_id(Uuid::new_v4())).unwrap();

        assert_eq!(true, actual.is_none());
    }

    #[test]
    fn upload_repository_save_should_insert_new_model() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let db = runtime.block_on(setup_test_database_connection());
        let repository = UploadRepository::new(db.clone());
        let upload = Upload::test();

        runtime.block_on(repository.save(upload.clone())).unwrap();
        let existing = runtime.block_on(upload::Entity::find_by_id(upload.id).one(&db)).unwrap();

        assert_eq!(true, existing.is_some());
        assert_eq!(upload, existing.unwrap().into());
    }

    async fn insert_test_upload(db: &DatabaseConnection) -> Upload {
        let model: upload::Model = Upload::test().into();
        let active_model: upload::ActiveModel = model.clone().into();
        active_model.insert(db).await.unwrap().into()
    }

    async fn insert_many_test_uploads(db: &DatabaseConnection, count: i8) -> Vec<Upload> {
        let mut uploads: Vec<Upload> = vec![];

        for _ in 0..count {
            uploads.push(insert_test_upload(db).await);
        }

        uploads
    }
}