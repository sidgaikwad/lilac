use k8s_openapi::api::rbac::v1::PolicyRule;

pub fn pod_read_policy() -> PolicyRule {
    PolicyRule {
        api_groups: Some(vec!["".into()]),
        non_resource_urls: None,
        resource_names: None,
        resources: Some(vec!["pods".into(), "pods/logs".into()]),
        verbs: vec!["get".into(), "list".into(), "watch".into()],
    }
}

pub fn pod_write_policy() -> PolicyRule {
    PolicyRule {
        api_groups: Some(vec!["".into()]),
        non_resource_urls: None,
        resource_names: None,
        resources: Some(vec!["pods".into(), "pods/logs".into()]),
        verbs: vec!["get".into(), "list".into(), "watch".into()],
    }
}