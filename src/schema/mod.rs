
// Re-export the SchemaOld model in a more friendly namesapce.
pub use schema_model::*;

pub use self::de::{from_str};

pub mod de;
pub mod error;
