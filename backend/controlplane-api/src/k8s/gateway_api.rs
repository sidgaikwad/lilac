use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "HTTPRoute",
    plural = "httproutes"
)]
#[kube(namespaced)]
pub struct HTTPRouteSpec {
    pub parent_refs: Option<Vec<ParentReference>>,
    pub hostnames: Option<Vec<String>>,
    pub rules: Option<Vec<HTTPRouteRule>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ParentReference {
    pub group: Option<String>,
    pub kind: Option<String>,
    pub name: String,
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteRule {
    pub matches: Option<Vec<HTTPRouteMatch>>,
    pub backend_refs: Option<Vec<HTTPBackendRef>>,
    pub filters: Option<Vec<HTTPRouteFilter>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteFilter {
    pub r#type: String,
    pub url_rewrite: Option<URLRewrite>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct URLRewrite {
    pub path: Option<PathRewrite>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PathRewrite {
    pub r#type: String,
    pub replace_prefix_match: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteMatch {
    pub path: Option<HTTPPathMatch>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPPathMatch {
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPBackendRef {
    pub name: String,
    pub port: u16,
}
