use axum::{
    extract::{Path, State},
    Json,
};

use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    auth::claims::Claims,
    data_sources::{check_s3_bucket_access, check_snowflake_access, get_bucket_location},
    database::Database,
    model::{
        dataset::{Dataset, DatasetId, DatasetSource, S3Bucket, SnowflakeConnector},
        project::ProjectId,
    },
    ServiceError,
};

#[derive(Serialize, Debug)]
pub struct DatasetSummary {
    pub id: DatasetId,
    pub name: String,
    pub description: Option<String>,
    pub dataset_source: String,
}

#[derive(Serialize, Debug)]
pub struct ListDatasetsResponse {
    datasets: Vec<DatasetSummary>,
}

#[instrument(level = "info", skip_all, ret, err)]
pub async fn list_datasets_handler(
    _claims: Claims,
    State(db): State<Database>,
    Path(project_id): Path<ProjectId>,
) -> Result<Json<ListDatasetsResponse>, ServiceError> {
    let datasets = db.list_datasets(&project_id).await?;
    Ok(Json(ListDatasetsResponse {
        datasets: datasets
            .into_iter()
            .map(|v| DatasetSummary {
                id: v.dataset_id,
                name: v.dataset_name,
                description: v.description,
                dataset_source: v.dataset_source.get_type(),
            })
            .collect(),
    }))
}

#[instrument(level = "info", skip(_claims, db, request), ret, err)]
pub async fn connect_data_source(
    _claims: Claims,
    State(db): State<Database>,
    Path(project_id): Path<ProjectId>,
    Json(request): Json<CreateDatasetRequest>,
) -> Result<Json<CreateDatasetResponse>, ServiceError> {
    // verify project exists
    let _project = db.get_project(&project_id).await?;

    let dataset_source = match request.source {
        DatasetSourceRequest::Unknown => {
            return Err(ServiceError::BadRequest("unknown source".into()))
        }
        DatasetSourceRequest::S3 {
            bucket_name,
            access_key,
            secret_key,
        } => {
            let secret_key = secret_key.into();
            let region = get_bucket_location(&bucket_name, &access_key, &secret_key).await?;
            let s3_bucket = S3Bucket::new(
                bucket_name,
                access_key,
                secret_key,
                region.to_string(),
            );
            check_s3_bucket_access(&s3_bucket).await?;
            DatasetSource::S3(s3_bucket)
        }
        DatasetSourceRequest::Snowflake {
            username,
            password,
            account,
            warehouse,
            database,
            schema,
            role,
        } => {
            let password = password.into();
            let snowflake_connector = SnowflakeConnector::new(
                username, password, account, warehouse, database, schema, role,
            );
            check_snowflake_access(&snowflake_connector).await?;
            DatasetSource::Snowflake(snowflake_connector)
        }
    };

    let dataset = Dataset::create(
        request.dataset_name,
        request.description,
        project_id,
        dataset_source,
    );
    let id = db.create_dataset(dataset).await?;
    Ok(Json(CreateDatasetResponse { id }))
}

#[instrument(level = "info", skip(_claims, db, request), ret, err)]
pub async fn test_data_source_connection(
    _claims: Claims,
    State(db): State<Database>,
    Path(project_id): Path<ProjectId>,
    Json(request): Json<CreateDatasetRequest>,
) -> Result<Json<TestDatasetResponse>, ServiceError> {
    // verify project exists
    let _project = db.get_project(&project_id).await?;

    let result = match request.source {
        DatasetSourceRequest::Unknown => {
            return Err(ServiceError::BadRequest("unknown source".into()))
        }
        DatasetSourceRequest::S3 {
            bucket_name,
            access_key,
            secret_key,
        } => {
            let secret_key = secret_key.into();
            let region = get_bucket_location(&bucket_name, &access_key, &secret_key).await?;
            let s3_bucket = S3Bucket::new(
                bucket_name,
                access_key,
                secret_key,
                region.to_string(),
            );
            check_s3_bucket_access(&s3_bucket).await
        }
        DatasetSourceRequest::Snowflake {
            username,
            password,
            account,
            warehouse,
            database,
            schema,
            role,
        } => {
            let password = password.into();
            let snowflake_connector = SnowflakeConnector::new(
                username, password, account, warehouse, database, schema, role,
            );
            check_snowflake_access(&snowflake_connector).await
        }
    };
    Ok(Json(TestDatasetResponse {
        success: result.is_ok(),
    }))
}

#[derive(Serialize, Debug)]
pub struct TestDatasetResponse {
    success: bool,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(tag = "source_type")]
pub enum DatasetSourceRequest {
    #[default]
    Unknown,
    S3 {
        bucket_name: String,
        access_key: String,
        secret_key: String,
    },
    Snowflake {
        username: String,
        password: String,
        account: String,
        warehouse: Option<String>,
        database: Option<String>,
        schema: Option<String>,
        role: Option<String>,
    },
}

#[derive(Deserialize, Debug)]
pub struct CreateDatasetRequest {
    dataset_name: String,
    description: Option<String>,
    source: DatasetSourceRequest,
}

#[derive(Serialize, Debug)]
pub struct CreateDatasetResponse {
    id: DatasetId,
}

#[instrument(level = "info", skip(_claims, db), ret, err)]
pub async fn get_dataset(
    _claims: Claims,
    State(db): State<Database>,
    Path(dataset_id): Path<DatasetId>,
) -> Result<Json<GetDatasetResponse>, ServiceError> {
    let dataset = db.get_dataset(&dataset_id).await?;

    Ok(Json(GetDatasetResponse {
        id: dataset.dataset_id,
        name: dataset.dataset_name,
        description: dataset.description,
        project_id: dataset.project_id,
        dataset_source: dataset.dataset_source,
    }))
}

#[derive(Serialize, Debug)]
pub struct GetDatasetResponse {
    id: DatasetId,
    name: String,
    description: Option<String>,
    project_id: ProjectId,
    dataset_source: DatasetSource,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn delete_dataset_handler(
    claims: Claims,
    State(db): State<Database>,
    Path(dataset_id): Path<DatasetId>,
) -> Result<(), ServiceError> {
    let _dataset = db.get_dataset(&dataset_id).await?;

    db.delete_dataset(&dataset_id).await?;

    Ok(())
}
