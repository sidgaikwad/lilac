use std::{future::Future, str};

use tokio::process::Command;

use crate::k8s::{K8sError, K8sWrapper};

const HELM_CMD: &'static str = "helm";

pub trait Helm {
    fn helm_install(
        &self,
        namespace: &str,
        name: &str,
        chart: &str,
    ) -> impl Future<Output = Result<(), K8sError>> + Send;

    fn helm_uninstall(
        &self,
        namespace: &str,
        name: &str,
    ) -> impl Future<Output = Result<(), K8sError>> + Send;

    fn helm_upgrade(
        &self,
        namespace: &str,
        name: &str,
        chart: &str,
    ) -> impl Future<Output = Result<(), K8sError>> + Send;
}

impl Helm for K8sWrapper {
    async fn helm_install(&self, namespace: &str, name: &str, chart: &str) -> Result<(), K8sError> {
        let namespace = shlex::try_quote(namespace).map_err(|e| K8sError::Helm(e.to_string()))?;
        let name = shlex::try_quote(name).map_err(|e| K8sError::Helm(e.to_string()))?;
        let chart = shlex::try_quote(chart).map_err(|e| K8sError::Helm(e.to_string()))?;
        let helm_cmd = Command::new(HELM_CMD)
            .args(&["install", &name, &chart, "--namespace", &namespace])
            .output()
            .await
            .map_err(|e| {
                tracing::error!("error running helm install: {e}");
                K8sError::Helm(e.to_string())
            })?;
        match helm_cmd.status.success() {
            true => Ok(()),
            false => {
                tracing::error!("helm command failed: {}", str::from_utf8(&helm_cmd.stderr).expect("output to be utf-8"));
                Err(K8sError::Helm(format!(
                    "helm install failed with exit code: {}",
                    helm_cmd.status.code().unwrap_or(-1)
                )))
            },
        }
    }

    async fn helm_uninstall(&self, namespace: &str, name: &str) -> Result<(), super::K8sError> {
        let namespace = shlex::try_quote(namespace).map_err(|e| K8sError::Helm(e.to_string()))?;
        let name = shlex::try_quote(name).map_err(|e| K8sError::Helm(e.to_string()))?;
        let helm_cmd = Command::new(HELM_CMD)
            .args(&["uninstall", &name, "--namespace", &namespace])
            .output()
            .await
            .map_err(|e| {
                tracing::error!("error running helm uninstall: {e}");
                K8sError::Helm(e.to_string())
            })?;
        match helm_cmd.status.success() {
            true => Ok(()),
            false => Err(K8sError::Helm(format!(
                "helm uninstall failed with exit code: {}",
                helm_cmd.status.code().unwrap_or(-1)
            ))),
        }
    }

    async fn helm_upgrade(
        &self,
        namespace: &str,
        name: &str,
        chart: &str,
    ) -> Result<(), super::K8sError> {
        let namespace = shlex::try_quote(namespace).map_err(|e| K8sError::Helm(e.to_string()))?;
        let name = shlex::try_quote(name).map_err(|e| K8sError::Helm(e.to_string()))?;
        let chart = shlex::try_quote(chart).map_err(|e| K8sError::Helm(e.to_string()))?;
        let helm_cmd = Command::new(HELM_CMD)
            .args(&["upgrade", &name, &chart, "--namespace", &namespace])
            .output()
            .await
            .map_err(|e| {
                tracing::error!("error running helm upgrade: {e}");
                K8sError::Helm(e.to_string())
            })?;
        match helm_cmd.status.success() {
            true => Ok(()),
            false => Err(K8sError::Helm(format!(
                "helm upgrade failed with exit code: {}",
                helm_cmd.status.code().unwrap_or(-1)
            ))),
        }
    }
}
