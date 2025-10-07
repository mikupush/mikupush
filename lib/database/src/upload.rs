use crate::error::DbError;
use crate::schema::uploads as uploads_table;
use crate::DbPool;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use mikupush_common::Upload;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::uploads)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UploadModel {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub mime_type: String,
    pub path: String,
    pub url: String,
    pub server_id: String,
    pub created_at: NaiveDateTime,
    pub status: String,
}

impl TryFrom<UploadModel> for Upload {
    type Error = DbError;

    fn try_from(model: UploadModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(model.id.as_str())?,
            name: model.name,
            size: model.size as u64,
            mime_type: model.mime_type,
            path: model.path,
            url: model.url,
            created_at: model.created_at.and_utc(),
            status: model.status.into(),
            server_id: Uuid::parse_str(model.server_id.as_str())?
        })
    }
}

impl From<Upload> for UploadModel {
    fn from(model: Upload) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name,
            size: model.size as i64,
            mime_type: model.mime_type,
            path: model.path,
            url: model.url,
            created_at: model.created_at.naive_utc(),
            status: model.status.to_string(),
            server_id: model.server_id.to_string()
        }
    }
}

pub struct UploadRepository {
    connection_pool: DbPool,
}

impl UploadRepository {
    pub fn new(db: DbPool) -> Self {
        Self { connection_pool: db }
    }

    pub fn find_all(&self) -> Result<Vec<Upload>, DbError> {
        let mut connection = self.connection_pool.get()?;
        let entities = uploads_table::table
            .select(UploadModel::as_select())
            .load::<UploadModel>(&mut connection)?;

        let models: Vec<Upload> = entities
            .iter()
            .map(|entity| entity.clone().try_into())
            .filter_map(Result::ok)
            .collect();

        Ok(models)
    }

    pub fn find_by_id(&self, id: Uuid) -> Result<Option<Upload>, DbError> {
        let mut connection = self.connection_pool.get()?;
        let entity = uploads_table::table
            .find(id.to_string())
            .first::<UploadModel>(&mut connection)
            .optional()?;

        let entity = entity
            .map(|entity| entity.try_into().ok())
            .unwrap_or(None);

        Ok(entity)
    }

    pub fn save(&self, upload: Upload) -> Result<(), DbError> {
        let existing = self.find_by_id(upload.id)?;
        let model: UploadModel = upload.into();

        if existing.is_none() {
            self.insert(&model)?;
        } else {
            self.update(&model)?;
        }

        Ok(())
    }

    fn insert(&self, model: &UploadModel) -> Result<(), DbError> {
        let mut connection = self.connection_pool.get()?;
        diesel::insert_into(uploads_table::table)
            .values(model)
            .execute(&mut connection)?;
        Ok(())
    }

    fn update(&self, model: &UploadModel) -> Result<(), DbError> {
        let mut connection = self.connection_pool.get()?;
        diesel::update(uploads_table::table)
            .filter(uploads_table::id.eq(&model.id))
            .set(model)
            .execute(&mut connection)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::DbPool;
    use crate::tests::test_database_connection;
    use mikupush_common::Upload;
    use serial_test::serial;
    use uuid::Uuid;

    #[test]
    #[serial]
    fn upload_repository_find_all_should_return_all() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = UploadRepository::new(db.clone());
        let expected: Vec<Upload> = insert_many_test_uploads(&db, 10);

        let actual: Vec<Upload> = repository.find_all().unwrap();

        assert_eq!(expected.len(), actual.len());
    }

    #[test]
    #[serial]
    fn upload_repository_find_by_id_should_find_existing() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = UploadRepository::new(db.clone());
        let expected: Upload = insert_test_upload(&db);

        let actual: Option<Upload> = repository.find_by_id(expected.id).unwrap();

        assert_eq!(true, actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    #[serial]
    fn upload_repository_find_by_id_should_not_find_not_existing() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = UploadRepository::new(db.clone());

        let actual: Option<Upload> = repository.find_by_id(Uuid::new_v4()).unwrap();

        assert_eq!(true, actual.is_none());
    }

    #[test]
    #[serial]
    fn upload_repository_save_should_insert_new_model() {
        let db = test_database_connection();
        let mut connection = db.get().unwrap();
        clean(&mut connection);

        let repository = UploadRepository::new(db.clone());
        let upload = Upload::test();

        repository.save(upload.clone()).unwrap();
        let existing = find_by_id(upload.id, &mut connection);

        assert_eq!(true, existing.is_some());
        assert_eq!(upload, existing.unwrap().try_into().unwrap());
    }

    fn insert_test_upload(db: &DbPool) -> Upload {
        let mut connection = db.get().unwrap();
        let model = Upload::test();
        let result = diesel::insert_into(uploads_table::table)
            .values::<UploadModel>(model.clone().into())
            .execute(&mut connection);
        println!("insert test upload result: {:?}", result);
        model
    }

    fn insert_many_test_uploads(db: &DbPool, count: i8) -> Vec<Upload> {
        let mut uploads: Vec<Upload> = vec![];

        for _ in 0..count {
            uploads.push(insert_test_upload(db));
        }

        uploads
    }

    fn find_by_id(id: Uuid, connection: &mut SqliteConnection) -> Option<UploadModel> {
        uploads_table::table
            .find(id.to_string())
            .first::<UploadModel>(connection)
            .optional()
            .unwrap()
    }

    fn clean(connection: &mut SqliteConnection) {
        let _ = diesel::delete(uploads_table::table)
            .execute(connection);
    }
}