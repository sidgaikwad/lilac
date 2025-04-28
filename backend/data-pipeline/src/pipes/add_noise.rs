use crate::pipe_core::{ImagePipe, PipeDefinition, PipeError, PipeImageData};
use async_trait::async_trait;
use common::model::step_definition::{StepCategory, StepDefinition, StepType};
use image::DynamicImage;
use imageproc::noise;
// Removed unused imports: Rng, SeedableRng, StdRng
use rand; // Keep top-level rand for rand::random
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::{Display, EnumString};
use uuid::uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NoiseType {
    Gaussian,
}

#[derive(Debug, Clone)]
pub struct AddNoisePipe {
    noise_type: NoiseType,
    mean: f64,
    std_dev: f64,
    seed: Option<u64>,
}

impl AddNoisePipe {
    pub fn new_gaussian(mean: f64, std_dev: f64, seed: Option<u64>) -> Result<Self, String> {
        if std_dev < 0.0 {
            return Err("Standard deviation must be non-negative.".to_string());
        }
        Ok(Self {
            noise_type: NoiseType::Gaussian,
            mean,
            std_dev,
            seed,
        })
    }

    fn apply_noise(&self, img: &DynamicImage) -> DynamicImage {
        let seed_val: u64 = self.seed.unwrap_or_else(rand::random);

        match self.noise_type {
            NoiseType::Gaussian => match img {
                DynamicImage::ImageLuma8(buf) => {
                    let mut noisy_buf = buf.clone();
                    noise::gaussian_noise_mut(&mut noisy_buf, self.mean, self.std_dev, seed_val);
                    DynamicImage::ImageLuma8(noisy_buf)
                }
                DynamicImage::ImageLumaA8(buf) => {
                    let mut noisy_buf = buf.clone();
                    noise::gaussian_noise_mut(&mut noisy_buf, self.mean, self.std_dev, seed_val);
                    DynamicImage::ImageLumaA8(noisy_buf)
                }
                DynamicImage::ImageRgb8(buf) => {
                    let mut noisy_buf = buf.clone();
                    noise::gaussian_noise_mut(&mut noisy_buf, self.mean, self.std_dev, seed_val);
                    DynamicImage::ImageRgb8(noisy_buf)
                }
                DynamicImage::ImageRgba8(buf) => {
                    let mut noisy_buf = buf.clone();
                    noise::gaussian_noise_mut(&mut noisy_buf, self.mean, self.std_dev, seed_val);
                    DynamicImage::ImageRgba8(noisy_buf)
                }
                _ => {
                    img.clone()
                }
            },
        }
    }
}

impl PipeDefinition for AddNoisePipe {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            id: uuid!("0a1b2c3d-4e5f-4a7b-8c9d-0e1f2a3b4c5d").into(),
            step_type: StepType::AddNoise,
            name: "Add Noise".into(),
            description: Some("Adds noise to the images.".into()),
            category: StepCategory::ImageProcessing,
            inputs: vec!["input".into()],
            outputs: vec!["output".into()],
            schema: json!({
                "type": "object",
                "properties": {
                    "noise_type": { "type": "string", "enum": ["gaussian"] },
                    "mean": { "type": "number" },
                    "std_dev": { "type": "number", "minimum": 0.0 },
                    "seed": { "type": ["integer", "null"] }
                },
                "required": ["noise_type"],
                 "allOf": [
                    {
                        "if": { "properties": { "noise_type": { "const": "gaussian" } } },
                        "then": { "required": ["mean", "std_dev"] }
                    },
                ]
            }),
        }
    }
}

#[async_trait]
impl ImagePipe for AddNoisePipe {
    fn name(&self) -> &'static str {
        "AddNoise"
    }

    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        let noise_params = self.clone();

        let output_batch: Vec<PipeImageData> = image_batch
            .into_par_iter()
            .map(move |mut img_data| {
                img_data.image = noise_params.apply_noise(&img_data.image);
                img_data
            })
            .collect();

        Ok(output_batch)
    }
}