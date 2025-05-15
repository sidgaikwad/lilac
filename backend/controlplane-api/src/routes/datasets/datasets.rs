use std::io::Cursor;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use common::{
    database::Database,
    model::{
        dataset::{Dataset, DatasetFile, DatasetFileMetadata, DatasetId},
        project::ProjectId,
    },
    s3::S3Wrapper,
    ServiceError,
};
use data_url::DataUrl;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{auth::claims::Claims, AppState};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatasetSummary {
    pub dataset_id: DatasetId,
    pub dataset_name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
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
        datasets: datasets.into_iter().map(|v| DatasetSummary {
            dataset_id: v.dataset_id,
            dataset_name: v.dataset_name,
            description: v.description,
        }).collect(),
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
    let project = db.get_project(&project_id).await?;
    let dataset_id = DatasetId::generate();
    let s3_path = s3.get_dataset_s3_path(&project.organization_id, &project_id, &dataset_id);
    let dataset = Dataset::new(
        dataset_id,
        request.dataset_name,
        request.description,
        project_id,
        s3_path.clone(),
    );

    let images = request
        .images
        .into_iter()
        .map(|v| {
            let url = DataUrl::process(&v.contents).unwrap();
            let (body, _fragment) = url.decode_to_vec().unwrap();

            DatasetFile::new(
                v.metadata.file_name,
                v.metadata.file_type,
                v.metadata.size,
                v.metadata.created_at,
                "".to_string(),
                body,
            )
        })
        .collect();
    s3.upload_files(&s3_path, images).await?;

    let id = db.create_dataset(dataset).await?;
    Ok(Json(CreateDatasetResponse { id }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct FileMetaadataRequest {
    pub file_name: String,
    pub file_type: String,
    pub size: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRequest {
    pub metadata: FileMetaadataRequest,
    pub contents: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatasetRequest {
    dataset_name: String,
    description: Option<String>,
    images: Vec<FileRequest>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatasetResponse {
    id: DatasetId,
}


#[instrument(level = "info", skip(_claims, db, s3), ret, err)]
pub async fn get_dataset(
    _claims: Claims,
    State(db): State<Database>,
    State(s3): State<S3Wrapper>,
    Path((project_id, dataset_id)): Path<(ProjectId, DatasetId)>,
) -> Result<Json<GetDatasetResponse>, ServiceError> {
    let dataset = db.get_dataset(&dataset_id).await?;

    let files = s3.list_dataset_files(&dataset.dataset_path).await?;
    
    Ok(Json(GetDatasetResponse { 
        files,
     }))
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDatasetResponse {
    files: Vec<DatasetFileMetadata>,
}