use std::{sync::Arc, time::Duration};

use common::{database::Database, model::{jobs::Job, step_definition::StepType}, ServiceError};
use data_pipeline::{get_steps_to_register, pipe_core::ImagePipe, pipes::{blur::{BlurDetectionMethod, BlurDetectorPipe}, noop::NoOpStep, resolution::ResolutionStandardizerPipe}};
use dotenv::dotenv;
use image::imageops::FilterType;
use jsonschema::validate;
use tokio::time::sleep;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing::info!("starting app");
    // load .env
    dotenv().ok();

    tracing::info!("dotenv loaded");

    // initialize tracing
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL to be set");

    tracing::info!("database url: {}", db_url);
    let db = Arc::new(Database::new(&db_url).await.expect("database to connect"));
    db.migrate().await.expect("migrations to complete");

    let step_definitions = get_steps_to_register();
    for step_definition in step_definitions {
        jsonschema::validator_for(&step_definition.schema).expect("json schema to be valid");
        db.register_step_definition(step_definition)
            .await
            .expect("step to be registered");
    }

    tokio::spawn(process_jobs(db));
}

async fn process_jobs(db: Arc<Database>) {
    loop {
        let result = db.get_pending_job().await;

        match result {
            Ok(value) => {
                if let Some(job) = value {
                    let res = handle_job(db.clone(), &job).await;
                    if let Err(err) = res {
                        tracing::error!("error processing job: {err}");
                        db.fail_job(&job.job_id).await;
                    } else {
                        db.complete_job(&job.job_id).await;
                    }
                }
            }
            Err(err) => tracing::error!("error getting pending job: {err}"),
        }
        sleep(Duration::from_secs(10)).await;
    }
}

async fn handle_job(db: Arc<Database>, job: &Job) -> Result<(), ServiceError> {
    let pipeline = db.get_pipeline(&job.pipeline_id).await?;
    let mut pipes: Vec<Box<dyn ImagePipe>> = Vec::new();
    for step in &pipeline.steps {
        let step_definition = db.get_step_definition(&step.step_definition_id).await?;
        if let Err(err) = validate(&step_definition.schema, &step.step_parameters) {
            tracing::error!("schema validation failed: {err}");
            return Err(ServiceError::SchemaError)
        }
        let pipe: Box<dyn ImagePipe> = match step_definition.step_type {
            StepType::NoOp => Box::new(NoOpStep {}),
            StepType::BlurDetector => {
                // we can unwrap here because we already validated the schema
                let method = step.step_parameters.get("method").unwrap().as_str().unwrap();
                let method = match method {
                    "LaplacianVariance" => BlurDetectionMethod::LaplacianVariance,
                    "PixelVariance" => BlurDetectionMethod::PixelVariance,
                    "EdgeIntensity" => BlurDetectionMethod::EdgeIntensity,
                    "EdgeCount" => {
                        let emt = step.step_parameters.get("edge_magnitude_threshold").unwrap().as_u64().unwrap();
                        BlurDetectionMethod::EdgeCount { edge_magnitude_threshold: emt as u16 }
                    },
                    _ => Err(ServiceError::ParseError("method".to_string()))?,
                };
                let threshold = step.step_parameters.get("threshold").unwrap().as_f64().unwrap();
                Box::new(BlurDetectorPipe::new(method, threshold))
            },
            StepType::ResolutionStandardizer => {
                let target_width = step.step_parameters.get("target_width").unwrap().as_u64().unwrap();
                let target_height = step.step_parameters.get("target_height").unwrap().as_u64().unwrap();
                let filter_type = step.step_parameters.get("filter_type").unwrap().as_str().unwrap();
                let filter_type = match filter_type {
                    "Nearest" => FilterType::Nearest,
                    "Triangle" => FilterType::Triangle,
                    "CatmullRom" => FilterType::CatmullRom,
                    "Gaussian" => FilterType::Gaussian,
                    "Lanczos3" => FilterType::Lanczos3,
                    _ => Err(ServiceError::ParseError("filter_type".to_string()))?,
                };
                Box::new(ResolutionStandardizerPipe::new(target_width as u32, target_height as u32, filter_type))

            },
        };
        pipes.push(pipe);
    }

    // TODO: get source and destination from pipeline, and process image batch
    Ok(())
}
