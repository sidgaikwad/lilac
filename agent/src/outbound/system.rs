use async_trait::async_trait;
use nvml_wrapper::Nvml;
use sysinfo::{System};

use crate::domain::agent::{
    models::{CpuConfiguration, GpuConfiguration, NodeResources},
    ports::SystemMonitor,
};

pub struct SysinfoMonitor;

impl SysinfoMonitor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SystemMonitor for SysinfoMonitor {
    async fn get_node_resources(&self) -> anyhow::Result<NodeResources> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_count = sys.cpus().len();
        let cpu_config = CpuConfiguration {
            // Vendor ID is often not available in virtualized environments.
            manufacturer: sys.cpus().first().map_or("".to_string(), |c| c.vendor_id().to_string()),
            architecture: std::env::consts::ARCH.to_string(),
            // Report the total number of logical cores as millicores.
            millicores: (cpu_count * 1000) as u32,
        };

        let gpus = match Nvml::init() {
            Ok(nvml) => {
                let device_count = nvml.device_count()?;
                let mut gpu_configs = Vec::with_capacity(device_count as usize);
                for i in 0..device_count {
                    let device = nvml.device_by_index(i)?;
                    gpu_configs.push(GpuConfiguration {
                        // NVML is NVIDIA-specific.
                        manufacturer: "NVIDIA".to_string(),
                        model_name: device.name()?,
                        memory_mb: (device.memory_info()?.total / 1024 / 1024) as u32,
                    });
                }
                gpu_configs
            }
            Err(_) => {
                // This is a normal, expected condition on systems without NVIDIA GPUs
                // or without the NVML library installed. We simply report no GPUs.
                Vec::new()
            }
        };

        let resources = NodeResources {
            cpu: cpu_config,
            gpus,
            memory_mb: sys.total_memory() / 1024 / 1024,
        };

        Ok(resources)
    }
}