use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::time::Instant;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Record};
use tracing::{Id, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;
pub mod appmap_definition;
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AppMap {
    #[serde(flatten)]
    pub data: crate::appmap_definition::AppMapObject,
}
