use std::str;
use tokio::process::Command;

use crate::domain::integration::ports::K8sPortError;

pub async fn helm_install(
    namespace: &str,
    name: &str,
    chart: &str,
    values: Option<Vec<&str>>,
) -> Result<(), K8sPortError> {
    let chart = format!("oci://registry-1.docker.io/bitnamicharts/{chart}");
    let mut helm_cmd = Command::new("helm");

    helm_cmd.args(["install", name, &chart, "--namespace", namespace]);

    if let Some(vals) = values {
        for v in vals {
            helm_cmd.arg("--set");
            helm_cmd.arg(v);
        }
    }
    let output = helm_cmd
        .output()
        .await
        .map_err(|e| K8sPortError::Helm(e.to_string()))?;
    match output.status.success() {
        true => Ok(()),
        false => {
            tracing::error!(
                "helm command failed: {}",
                str::from_utf8(&output.stderr).expect("output to be utf-8")
            );
            Err(K8sPortError::Helm(format!(
                "helm install failed with exit code: {}",
                output.status.code().unwrap_or(-1)
            )))
        }
    }
}

pub async fn helm_uninstall(namespace: &str, name: &str) -> Result<(), K8sPortError> {
    let namespace = shlex::try_quote(namespace).map_err(|e| K8sPortError::Helm(e.to_string()))?;
    let name = shlex::try_quote(name).map_err(|e| K8sPortError::Helm(e.to_string()))?;
    let helm_cmd = Command::new("helm")
        .args(["uninstall", &name, "--namespace", &namespace])
        .output()
        .await
        .map_err(|e| K8sPortError::Helm(e.to_string()))?;
    match helm_cmd.status.success() {
        true => Ok(()),
        false => Err(K8sPortError::Helm(format!(
            "helm uninstall failed with exit code: {}",
            helm_cmd.status.code().unwrap_or(-1)
        ))),
    }
}
