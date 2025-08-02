use crate::{
    domain::agent::{
        models::{
            Architecture, Cpu, CpuManufacturer, Gpu, GpuManufacturer, GpuModel, NodeResources,
        },
        ports::SystemMonitor,
    },
    errors::SystemMonitorError,
};
use async_trait::async_trait;
use nvml_wrapper::Nvml;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use sysinfo::System;

pub struct HybridMonitor;

impl HybridMonitor {
    pub fn new() -> Self {
        Self
    }

    fn get_cpu_millicores() -> Result<i32, SystemMonitorError> {
        if Path::new("/sys/fs/cgroup/cpu.max").exists() {
            let cpu_max = fs::read_to_string("/sys/fs/cgroup/cpu.max")
                .map_err(|_| SystemMonitorError::ReadError)?
                .trim()
                .to_string();
            let parts: Vec<&str> = cpu_max.split_whitespace().collect();
            if parts.len() == 2 && parts[0] != "max" {
                let max_us: f64 = parts[0].parse().map_err(|_| SystemMonitorError::ReadError)?;
                let period_us: f64 = parts[1].parse().map_err(|_| SystemMonitorError::ReadError)?;
                return Ok(((max_us / period_us) * 1000.0) as i32);
            }
        }

        // Fallback for non-cgroup environments
        let mut sys = System::new();
        sys.refresh_cpu();
        Ok((sys.cpus().len() * 1000) as i32)
    }

    fn get_memory_mb() -> Result<i32, SystemMonitorError> {
        if Path::new("/sys/fs/cgroup/memory.max").exists() {
            let mem_max_bytes: i64 = fs::read_to_string("/sys/fs/cgroup/memory.max")
                .map_err(|_| SystemMonitorError::ReadError)?
                .trim()
                .parse()
                .map_err(|_| SystemMonitorError::ReadError)?;
            return Ok((mem_max_bytes / 1024 / 1024) as i32);
        }

        // Fallback for non-cgroup environments
        let mut sys = System::new();
        sys.refresh_memory();
        Ok((sys.total_memory() / 1024 / 1024) as i32)
    }
}

#[async_trait]
impl SystemMonitor for HybridMonitor {
    async fn get_node_resources(&self) -> Result<NodeResources, SystemMonitorError> {
        let mut sys = System::new();
        sys.refresh_cpu();

        let cpu_info = sys.cpus().first().ok_or(SystemMonitorError::ReadError)?;

        let cpu = Cpu {
            manufacturer: CpuManufacturer::from_str(cpu_info.vendor_id())
                .unwrap_or(CpuManufacturer::Intel),
            architecture: Architecture::from_str(std::env::consts::ARCH)
                .unwrap_or(Architecture::X86_64),
            millicores: Self::get_cpu_millicores()?,
        };

        let gpus = match Nvml::init() {
            Ok(nvml) => {
                let device_count =
                    nvml.device_count().map_err(|_| SystemMonitorError::ReadError)?;
                let mut gpu_configs = Vec::with_capacity(device_count as usize);
                for i in 0..device_count {
                    let device = nvml.device_by_index(i).map_err(|_| SystemMonitorError::ReadError)?;
                    let model_name = device.name().map_err(|_| SystemMonitorError::ReadError)?;
                    gpu_configs.push(Gpu {
                        manufacturer: GpuManufacturer::Nvidia,
                        model: GpuModel::from_str(&model_name).unwrap_or(GpuModel::A100),
                        count: 1,
                        memory_mb: (device
                            .memory_info()
                            .map_err(|_| SystemMonitorError::ReadError)?
                            .total
                            / 1024
                            / 1024) as i32,
                    });
                }
                gpu_configs
            }
            Err(_) => Vec::new(),
        };

        let resources = NodeResources {
            cpu,
            gpus,
            memory_mb: Self::get_memory_mb()?,
        };

        Ok(resources)
    }
}