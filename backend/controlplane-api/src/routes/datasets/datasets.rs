use axum::{
    extract::{Path, Query, State},
    Json,
};
use common::{
    aws::{get_s3_client_with_role, S3Wrapper}, database::Database, model::{
        dataset::{Dataset, DatasetId, DatasetSource},
        project::ProjectId,
    }, ServiceError
};

use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::auth::claims::Claims;

#[derive(Serialize, Debug)]
pub struct DatasetSummary {
    pub id: DatasetId,
    pub name: String,
    pub description: Option<String>,
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
            })
            .collect(),
    }))
}

#[instrument(level = "info", skip(_claims, db, s3, request), ret, err)]
pub async fn create_dataset(
    _claims: Claims,
    State(db): State<Database>,
    State(s3): State<S3Wrapper>,
    Path(project_id): Path<ProjectId>,
    Json(request): Json<CreateDatasetRequest>,
) -> Result<Json<CreateDatasetResponse>, ServiceError> {
    // verify project exists
    let _project = db.get_project(&project_id).await?;

    let dataset_source = match request.source {
        DatasetSourceRequest::Unknown => return Err(ServiceError::BadRequest("unknown source".into())),
        DatasetSourceRequest::S3 { bucket_name } => {
            let region = s3.get_bucket_location(&bucket_name).await?;
            DatasetSource::S3 { bucket_name: bucket_name, region: region }
        },
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


#[derive(Clone, Debug, Deserialize, Default)]
#[serde(tag = "source_type")]
pub enum DatasetSourceRequest {
    #[default]
    Unknown,
    S3 {
        bucket_name: String,
    }
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

#[instrument(level = "info", skip_all, fields(dataset_id = dataset_id.to_string(), user_id = _claims.sub.to_string()), err)]
pub async fn list_dataset_s3_folders(
    _claims: Claims,
    State(db): State<Database>,
    State(s3): State<S3Wrapper>,
    Path(dataset_id): Path<DatasetId>,
    Query(request): Query<ListS3ObjectsRequest>,
) -> Result<Json<ListS3ObjectsResponse>, ServiceError> {
    let dataset = db.get_dataset(&dataset_id).await?;
    let project = db.get_project(&dataset.project_id).await?;
    match dataset.dataset_source {
        DatasetSource::S3 { bucket_name, region } => {
            let aws_integration = project.aws_integration.ok_or(ServiceError::BadRequest("AWS integration not configured".into()))?;
            let s3_client = get_s3_client_with_role(&region, aws_integration.role_arn(), aws_integration.external_id()).await;
            let start_key_ref = request.start_after_key.as_ref().map(|v| v.as_str());
            let (prefixes, objects) = s3
                .list_dataset_prefixes(
                    Some(&s3_client),
                    &bucket_name,
                    &request.prefix,
                    start_key_ref,
                )
                .await?;

            Ok(Json(ListS3ObjectsResponse {
                prefixes: prefixes
                    .into_iter()
                    .filter_map(|v| {
                        v.prefix
                    })
                    .collect(),
                objects: objects
                    .into_iter()
                    .filter_map(|v| {
                        v.key
                    })
                    .collect(),
            }))
        }
        _ => {
            return Err(ServiceError::BadRequest(
                "dataset is not an S3 dataset".into(),
            ))
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ListS3ObjectsRequest {
    #[allow(dead_code)]
    prefix: String,
    #[allow(dead_code)]
    start_after_key: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ListS3ObjectsResponse {
    prefixes: Vec<String>,
    objects: Vec<String>,
}
