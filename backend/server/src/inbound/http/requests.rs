use serde::Deserialize;

// This struct is for the HTTP layer only.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateUserHttpRequestBody {
    pub name: String,
    pub email: String,
}

// This struct is for the HTTP layer only.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateProjectHttpRequestBody {
    pub name: String,
}
