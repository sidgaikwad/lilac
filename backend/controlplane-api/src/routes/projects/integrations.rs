use axum::{
    extract::{Path, State},
    Json,
};
use common::{
    aws::STSWrapper,
    database::Database,
    model::{integration::AWSIntegration, project::ProjectId},
    ServiceError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db, sts), ret, err)]
pub async fn set_aws_integration(
    claims: Claims,
    State(db): State<Database>,
    State(sts): State<STSWrapper>,
    Path(project_id): Path<ProjectId>,
    Json(request): Json<SetAWSAccessInfoRequest>,
) -> Result<Json<SetAWSAccessInfoResponse>, ServiceError> {
    let SetAWSAccessInfoRequest {
        role_arn,
        placeholder_external_id,
    } = request;

    let project = db.get_project(&project_id).await?;

    // verify credentials are usable with placeholder external id
    let _credentials = sts.assume_role(&role_arn, &placeholder_external_id).await?;

    let aws_integration = AWSIntegration::create(role_arn);
    db.set_project_aws_integration(&project.project_id, &aws_integration)
        .await?;

    // return actual generated external id
    Ok(Json(SetAWSAccessInfoResponse {
        external_id: aws_integration.external_id().clone(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct SetAWSAccessInfoRequest {
    #[allow(dead_code)]
    role_arn: String,
    #[allow(dead_code)]
    placeholder_external_id: String,
}

#[derive(Debug, Serialize)]
pub struct SetAWSAccessInfoResponse {
    external_id: String,
}
