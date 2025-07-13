#[derive(Clone, Debug, strum::Display)]
pub enum Role {
    #[strum(serialize = "owner")]
    Owner,
    #[strum(serialize = "admin")]
    Admin,
    #[strum(serialize = "member")]
    Member,
}
