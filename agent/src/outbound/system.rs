use async_trait::async_trait;
use nvml_wrapper::Nvml;
use sysinfo::{System};

use crate::domain::agent::{
    models::{
        Architecture, Cpu, CpuManufacturer, Gpu, GpuManufacturer, GpuModel, NodeResources,
    },
    ports::SystemMonitor,
};
use std::str::FromStr;

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
        let cpu_info = sys.cpus().first().unwrap();

        let cpu = Cpu {
            manufacturer: CpuManufacturer::from_str(cpu_info.vendor_id()).unwrap_or(CpuManufacturer::Intel),
            architecture: Architecture::from_str(std::env::consts::ARCH).unwrap_or(Architecture::X86_64),
            millicores: (cpu_count * 1000) as i32,
        };

        let gpus = match Nvml::init() {
            Ok(nvml) => {
                let device_count = nvml.device_count()?;
                let mut gpu_configs = Vec::with_capacity(device_count as usize);
                for i in 0..device_count {
                    let device = nvml.device_by_index(i)?;
                    let model_name = device.name()?;
                    gpu_configs.push(Gpu {
                        manufacturer: GpuManufacturer::Nvidia,
                        model: GpuModel::from_str(&model_name).unwrap_or(GpuModel::A100), // Default for now
                        count: 1,
                        memory_mb: (device.memory_info()?.total / 1024 / 1024) as i32,
                    });
                }
                gpu_configs
            }
            Err(_) => {
                Vec::new()
            }
        };

        let resources = NodeResources {
            cpu,
            gpus,
            memory_mb: (sys.total_memory() / 1024 / 1024) as i32,
        };

        Ok(resources)
    }
}