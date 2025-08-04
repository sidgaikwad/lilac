use secrecy::{ExposeSecret, SecretString};

pub mod auth;
pub mod cluster;
pub mod queue;
pub mod scheduler;
pub mod training_job;
pub mod user;

pub fn serialize_secret_string<S>(secret: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

#[macro_export]
macro_rules! identifier {
    ($struct_name:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
            Default,
        )]
        pub struct $struct_name(uuid::Uuid);

        impl $struct_name {
            pub fn new(id: uuid::Uuid) -> Self {
                Self(id)
            }

            pub fn generate() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            pub fn inner(&self) -> &uuid::Uuid {
                &self.0
            }

            pub fn into_inner(self) -> uuid::Uuid {
                self.0
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<uuid::Uuid> for $struct_name {
            fn from(id: uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl From<$struct_name> for uuid::Uuid {
            fn from(id: $struct_name) -> Self {
                id.0
            }
        }

        impl TryFrom<&str> for $struct_name {
            type Error = uuid::Error;

            fn try_from(id: &str) -> Result<Self, Self::Error> {
                Ok(Self(uuid::Uuid::try_parse(id)?))
            }
        }
    };
}
