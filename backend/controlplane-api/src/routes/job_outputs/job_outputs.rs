use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use common::{
    database::Database,
    model::jobs::JobId,
    ServiceError,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tracing::instrument;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobOutputSummary {
    job_id: JobId,
    input_dataset_name: Option<String>,
    completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListJobOutputsParams {
    project_id: Option<String>,
    organization_id: Option<String>,
}

#[instrument(level = "info", skip(_db), ret, err)]
pub async fn list_job_outputs_handler(
    State(_db): State<Database>,
    Query(_params): Query<ListJobOutputsParams>, 
) -> Result<Json<Vec<JobOutputSummary>>, ServiceError> {
    // let project_uuid = match params.project_id {
    //     Some(id_str) => Some(
    //         Uuid::try_parse(&id_str)
    //             .map_err(|_| ServiceError::ParseError("projectId".to_string()))?,
    //     ),
    //     None => None,
    // };

    let mut job_output_summaries = Vec::new();
    
    // TODO: (broken fix later todo) - Reinstate DB call and proper filtering
    // let completed_jobs_from_db = db.get_completed_job_outputs_by_project(project_uuid).await?;
    // for job_info in completed_jobs_from_db {
    //     let job_data_path = PathBuf::from("./data-pipeline/job_data")
    //         .join(job_info.job_id.inner().to_string())
    //         .join("output");
        
    //     if job_data_path.is_dir() {
    //         job_output_summaries.push(JobOutputSummary {
    //             job_id: job_info.job_id,
    //             input_dataset_name: job_info.input_dataset_name,
    //             completed_at: job_info.completed_at,
    //         });
    //     } else {
    //         tracing::warn!("Output directory not found for completed job {:?}: {:?}", job_info.job_id, job_data_path);
    //     }
    // }

    let base_job_data_path = PathBuf::from("./data-pipeline/job_data");
    if base_job_data_path.is_dir() {
        for entry_res in fs::read_dir(base_job_data_path).map_err(ServiceError::IoError)? {
            let entry = entry_res.map_err(ServiceError::IoError)?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(job_id_str) = path.file_name().and_then(|os_str| os_str.to_str()) {
                    if Uuid::try_parse(job_id_str).is_ok() {
                        let output_dir = path.join("output");
                        if output_dir.is_dir() {
                             match JobId::try_from(job_id_str.to_string()) {
                                Ok(job_id) => {
                                    job_output_summaries.push(JobOutputSummary {
                                        job_id,
                                        input_dataset_name: None, // Placeholder
                                        completed_at: None, 
                                    });
                                }
                                Err(e) => {
                                    tracing::warn!("Failed to parse directory name as JobId: {}: {}", job_id_str, e);
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        tracing::warn!("Base job_data directory not found: {:?}", base_job_data_path);
    }
    
    // Sort by job_id string representation for some consistency (optional)
    job_output_summaries.sort_by(|a, b| a.job_id.inner().to_string().cmp(&b.job_id.inner().to_string()));

    Ok(Json(job_output_summaries))
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobOutputImages {
    job_id: JobId,
    images: Vec<String>,
}

#[instrument(level = "info", skip_all, ret, err)]
pub async fn list_job_output_images_handler(
    Path(job_id_str): Path<String>,
) -> Result<Json<JobOutputImages>, ServiceError> {
    let job_id = JobId::try_from(job_id_str.clone())?;
    let output_path = PathBuf::from("./data-pipeline/job_data")
        .join(job_id_str)
        .join("output");
    
    let mut image_filenames = Vec::new();

    if output_path.is_dir() {
        for entry in fs::read_dir(&output_path).map_err(ServiceError::IoError)? {
            let entry = entry.map_err(ServiceError::IoError)?;
            let path = entry.path();
            if path.is_file() {
                if let Some(filename_osstr) = path.file_name() {
                    if let Some(filename) = filename_osstr.to_str() {
                        if filename.ends_with(".png")
                            || filename.ends_with(".jpg")
                            || filename.ends_with(".jpeg")
                        {
                            image_filenames.push(filename.to_string());
                        }
                    }
                }
            }
        }
    } else {
        tracing::warn!("Output directory not found for job {:?}: {:?}", job_id, output_path);

    }

    Ok(Json(JobOutputImages {
        job_id,
        images: image_filenames,
    }))
}