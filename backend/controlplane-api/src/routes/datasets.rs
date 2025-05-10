use axum::{extract::Path, routing::{get, post}, Extension, Json, Router};
use base64::{engine::general_purpose::URL_SAFE, prelude::BASE64_STANDARD, DecodeError, Engine};
use std::{fs, path::PathBuf};
use common::{database::Database, model::{dataset::{Dataset, DatasetId}, project::ProjectId}, ServiceError};
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};

use crate::auth::claims::Claims;

#[derive(Serialize, Debug)]
pub struct ListDatasetsResponse {
    datasets: Vec<String>,
}

#[instrument(level = "info", skip_all, ret, err)]
async fn list_datasets_handler() -> Result<Json<ListDatasetsResponse>, ServiceError> {
    let datasets_base_path = PathBuf::from("./data-pipeline/test_images");
    match datasets_base_path.canonicalize() {
        Ok(abs_path) => tracing::info!("Attempting to list datasets from path: {:?}", abs_path),
        Err(_) => tracing::info!("Attempting to list datasets from relative path: {:?}", datasets_base_path),
    }
    let mut dataset_names = Vec::new();

    if datasets_base_path.is_dir() {
        for entry in fs::read_dir(&datasets_base_path).map_err(|e| {
            tracing::error!("Failed to read datasets directory {:?}: {}", datasets_base_path, e);
            ServiceError::IoError(e)
        })? {
            let entry = entry.map_err(|e| {
                tracing::error!("Failed to read directory entry: {}", e);
                ServiceError::IoError(e)
            })?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    dataset_names.push(name.to_string());
                }
            }
        }
    } else {
        tracing::warn!("Datasets base path not found or not a directory: {:?}", datasets_base_path);
    }
    Ok(Json(ListDatasetsResponse { datasets: dataset_names }))
}


#[instrument(level = "info", skip(claims, db, request), ret, err)]
async fn create_dataset(
    claims: Claims,
    db: Extension<Database>,
    Path(project_id): Path<ProjectId>,
    Json(request): Json<CreateDatasetRequest>,
) -> Result<Json<CreateDatasetResponse>, ServiceError> {
    let images: Vec<Vec<u8>> = request.images.into_iter().map(|v| BASE64_STANDARD.decode(v)).collect::<Result<Vec<Vec<u8>>, DecodeError>>().map_err(|e| {
        error!("{e:?}");
        ServiceError::BadRequest("bad images".into())
    })?;
    let dir = format!("/usr/local/app/data-pipeline/test_images/{}", &request.dataset_name);
    info!("creating dir {dir}");
    fs::create_dir(dir.clone())?;
    for (i, image) in images.iter().enumerate() {
        info!("writing image {}", format!("{dir}/image_{}.png", i));
        fs::write(format!("{dir}/image_{}.png", i), image)?;
    }
    let dataset = Dataset::create(request.dataset_name, request.description, project_id, dir);
    let id = db.create_dataset(dataset).await?;
    Ok(Json(CreateDatasetResponse { id }))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct CreateDatasetRequest {
    dataset_name: String,
    description: Option<String>,
    images: Vec<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct CreateDatasetResponse {
    id: DatasetId
}

pub fn datasets_routes() -> Router {
    Router::new()
        .route("/", get(list_datasets_handler))
        .route("/{project_id}", post(create_dataset))
}