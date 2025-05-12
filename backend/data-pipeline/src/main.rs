use std::{path::PathBuf, str::FromStr, sync::Arc, time::Duration}; // Added PathBuf, FromStr

use common::{
    database::Database, model::{dataset::{Dataset, DatasetId}, jobs::Job, step_definition::StepType}, s3::S3Wrapper, ServiceError
};
use data_pipeline::{
    get_steps_to_register,
    pipe_core::ImagePipe,
    pipeline_definition::{DataDestination, DataSource, Pipeline},
    pipes::{
        // Import all pipes and necessary enums
        add_noise::{AddNoisePipe, NoiseType},
        blur::{BlurDetectionMethod, BlurPipe}, // Use BlurPipe
        brightness::BrightnessPipe,
        contrast::ContrastPipe,
        flip::{FlipDirection, FlipPipe},
        grayscale::GrayscalePipe,
        noop::NoOpStep,
        resize::ResizePipe, // Use ResizePipe
        rotate::{RotatePipe, RotationAngle},
    },
    runner::run_pipeline,
};
use dotenv::dotenv;
use futures::future::join_all;
use image::imageops::FilterType;
// Use full path for validate to avoid ambiguity if crate name changes
use jsonschema::validate;
use tokio::time::sleep;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

// Specific helpers for common types, handling potential errors during conversion
fn get_json_string<'a>(value_opt: Option<&'a serde_json::Value>, param_name: &str) -> Result<&'a str, ServiceError> {
    value_opt.ok_or_else(|| ServiceError::MissingParameter(param_name.to_string()))?
        .as_str()
        .ok_or_else(|| ServiceError::InvalidParameterType(param_name.to_string(), "Expected string".to_string()))
}

fn get_json_f64(value_opt: Option<&serde_json::Value>, param_name: &str) -> Result<f64, ServiceError> {
    value_opt.ok_or_else(|| ServiceError::MissingParameter(param_name.to_string()))?
        .as_f64()
        .ok_or_else(|| ServiceError::InvalidParameterType(param_name.to_string(), "Expected number".to_string()))
}

fn get_json_i64(value_opt: Option<&serde_json::Value>, param_name: &str) -> Result<i64, ServiceError> {
    value_opt.ok_or_else(|| ServiceError::MissingParameter(param_name.to_string()))?
        .as_i64()
        .ok_or_else(|| ServiceError::InvalidParameterType(param_name.to_string(), "Expected integer".to_string()))
}

fn get_json_u64(value_opt: Option<&serde_json::Value>, param_name: &str) -> Result<u64, ServiceError> {
    value_opt.ok_or_else(|| ServiceError::MissingParameter(param_name.to_string()))?
        .as_u64()
        .ok_or_else(|| ServiceError::InvalidParameterType(param_name.to_string(), "Expected positive integer".to_string()))
}


// Helper to get string and parse enum
fn get_json_enum<T>(
    value_opt: Option<&serde_json::Value>,
    param_name: &str,
) -> Result<T, ServiceError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = get_json_string(value_opt, param_name)?;
    T::from_str(s).map_err(|e| ServiceError::InvalidParameterValue(param_name.to_string(), e.to_string()))
}


#[tokio::main]
async fn main() {
    tracing::info!("starting app");
    dotenv().ok();
    tracing::info!("dotenv loaded");

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL to be set");

    tracing::info!("database url: {}", db_url);
    let db = Arc::new(Database::new(&db_url).await.expect("database to connect"));
    db.migrate().await.expect("migrations to complete");

    let bucket_name = std::env::var("CUSTOMER_ASSETS_BUCKET").expect("CUSTOMER_ASSETS_BUCKET to be set");
    let s3 = S3Wrapper::new_from_default(bucket_name).await;

    let step_definitions = get_steps_to_register();
    for step_definition in step_definitions {
        if let Err(e) = jsonschema::validator_for(&step_definition.schema) {
             tracing::error!(schema = ?step_definition.schema, error = %e, "Invalid JSON schema for step type {:?}", step_definition.step_type);
             panic!("Invalid schema for step type {:?}", step_definition.step_type);
        }
        db.register_step_definition(step_definition)
            .await
            .expect("step to be registered");
    }

    let handle = tokio::spawn(process_jobs(db, s3));
    join_all(vec![handle]).await;
}

async fn process_jobs(db: Arc<Database>, s3: S3Wrapper) {
    loop {
        let result = db.get_pending_job().await;

        match result {
            Ok(Some(job)) => {
                tracing::info!(job_id = %job.job_id.inner(), "Processing job");
                let res = handle_job(db.clone(), s3.clone(), &job).await;
                if let Err(err) = res {
                    tracing::error!(job_id = %job.job_id.inner(), error = %err, "Error processing job");
                    if let Err(e) = db.fail_job(&job.job_id).await {
                        tracing::error!(job_id = %job.job_id.inner(), error = %e, "Error failing job");
                    }
                } else {
                     tracing::info!(job_id = %job.job_id.inner(), "Job completed successfully");
                    if let Err(e) = db.complete_job(&job.job_id).await {
                        tracing::error!(job_id = %job.job_id.inner(), error = %e, "Error completing job");
                    }
                }
            }
            Ok(None) => { /* No job found, wait */ }
            Err(err) => tracing::error!("Error getting pending job: {}", err),
        }
        sleep(Duration::from_secs(10)).await;
    }
}

async fn handle_job(db: Arc<Database>, s3: S3Wrapper, job: &Job) -> Result<(), ServiceError> {
    let pipeline = db.get_pipeline(&job.pipeline_id).await?;
    let mut pipes: Vec<Box<dyn ImagePipe>> = Vec::new();

    for step_model in &pipeline.steps {
        let step_definition = db.get_step_definition(&step_model.step_definition_id).await?;

        // Validate parameters against schema *before* trying to parse them
        if let Err(error) = validate(&step_model.step_parameters, &step_definition.schema) { 
            tracing::error!(step_id = %step_model.step_id.inner(), schema = ?step_definition.schema, params = ?step_model.step_parameters, error=%error, "Schema validation failed");
            return Err(ServiceError::SchemaValidationError(error.to_string()));
        }

        let params = &step_model.step_parameters;

        // Instantiate pipe based on StepType
        let pipe: Box<dyn ImagePipe> = match step_definition.step_type {
            StepType::NoOp => Box::new(NoOpStep {}),
            StepType::BlurDetector => {
                // Use helper functions for parsing
                let method_str = get_json_string(params.get("method"), "method")?;
                let threshold = get_json_f64(params.get("threshold"), "threshold")?;

                let method = match method_str {
                    "LaplacianVariance" => BlurDetectionMethod::LaplacianVariance,
                    "PixelVariance" => BlurDetectionMethod::PixelVariance,
                    "EdgeIntensity" => BlurDetectionMethod::EdgeIntensity,
                    "EdgeCount" => {
                        let emt = get_json_u64(params.get("edge_magnitude_threshold"), "edge_magnitude_threshold")? as u16;
                        BlurDetectionMethod::EdgeCount { edge_magnitude_threshold: emt }
                    }
                    _ => return Err(ServiceError::InvalidParameterValue("method".to_string(), method_str.to_string())),
                };
                // Use BlurPipe struct, handle Result from new
                Box::new(BlurPipe::new(method, threshold).map_err(|e| ServiceError::BadRequest(e))?)
            }
            StepType::ResolutionStandardizer => {
                let target_width = get_json_u64(params.get("target_width"), "target_width")? as u32;
                let target_height = get_json_u64(params.get("target_height"), "target_height")? as u32;
                let filter_str = get_json_string(params.get("filter_type"), "filter_type")?;
                let filter_type = match filter_str {
                    "Nearest" => FilterType::Nearest,
                    "Triangle" => FilterType::Triangle,
                    "CatmullRom" => FilterType::CatmullRom,
                    "Gaussian" => FilterType::Gaussian,
                    "Lanczos3" => FilterType::Lanczos3,
                    _ => return Err(ServiceError::InvalidParameterValue("filter_type".to_string(), filter_str.to_string())),
                };
                // Use ResizePipe struct, handle Result from new
                Box::new(ResizePipe::new(target_width, target_height, filter_type).map_err(|e| ServiceError::BadRequest(e))?)
            }
             // --- Add arms for new pipes ---
             StepType::Rotate => {
                 let angle = get_json_enum::<RotationAngle>(params.get("angle"), "angle")?;
                 Box::new(RotatePipe::new(angle))
             }
             StepType::Flip => {
                 let direction = get_json_enum::<FlipDirection>(params.get("direction"), "direction")?;
                 Box::new(FlipPipe::new(direction))
             }
             StepType::Grayscale => {
                 Box::new(GrayscalePipe::new()) // No params
             }
             StepType::Brightness => {
                 let value = get_json_i64(params.get("value"), "value")? as i32;
                 Box::new(BrightnessPipe::new(value))
             }
             StepType::Contrast => {
                 let value = get_json_f64(params.get("contrast"), "contrast")? as f32;
                 Box::new(ContrastPipe::new(value))
             }
             StepType::AddNoise => {
                 let noise_type = get_json_enum::<NoiseType>(params.get("noise_type"), "noise_type")?;
                 match noise_type {
                     NoiseType::Gaussian => {
                         let mean = get_json_f64(params.get("mean"), "mean")?;
                         let std_dev = get_json_f64(params.get("std_dev"), "std_dev")?;
                         let seed = params.get("seed").map(|v| v.as_u64()).flatten();
                         // Handle Result from new_gaussian
                         Box::new(AddNoisePipe::new_gaussian(mean, std_dev, seed).map_err(|e| ServiceError::BadRequest(e))?)
                     }
                     // Add other noise types later if needed
                 }
             }
            // Handle Unknown or other unimplemented types
            StepType::Unknown => {
                 tracing::warn!("Encountered Unknown StepType");
                 // Defaulting to NoOp, could also error out
                 Box::new(NoOpStep {})
            }
            // The _ arm was removed as all StepType variants are explicitly handled.
            // StepType::Unknown serves as the catch-all for any new variants not yet implemented.
        };
        pipes.push(pipe);
    }

    let dataset = db.get_dataset(&job.input_dataset_id).await?;
    let project = db.get_project(&dataset.project_id).await?;

    let output_dataset_id = DatasetId::generate();
    let output_path = s3.get_dataset_s3_path(&project.organization_id, &project.project_id, &output_dataset_id);
    let output_dataset = db.create_dataset(Dataset::new(output_dataset_id, format!("{}-output", dataset.dataset_name), Some(format!("The results from running pipeline \"{}\" on dataset \"{}\"", pipeline.pipeline_name, dataset.dataset_name)), dataset.project_id, output_path)).await?;

    let output_path = PathBuf::from(format!("./job_data/{}/output", job.job_id.inner()));
    std::fs::create_dir_all(&output_path).map_err(|e| ServiceError::IoError(e))?;

    let pl = Pipeline {
        id: pipeline.pipeline_id.inner().to_string(),
        name: pipeline.pipeline_name.clone(),
        input_source: DataSource::S3Path(s3.clone(), dataset.dataset_path),
        output_destination: DataDestination::Dataset(output_dataset),
        stages: pipes,
    };

    run_pipeline(db.clone(), s3.clone(), &pl)
        .await
        .map_err(|e| {
             tracing::error!("Pipeline execution failed: {:?}", e);
             ServiceError::PipelineExecutionError(e.to_string())
        })?;

    Ok(())
}
