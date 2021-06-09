use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// The attachment ID.
    id: Snowflake,
    /// Name of the attached file.
    filename: String,
    /// The attachment's media type.
    content_type: Option<String>,
    /// The size of the file in bytes.
    size: usize,
    /// The source URL of the file.
    url: String,
    /// A proxied URL of the file.
    proxy_url: String,
    /// Height of the file (if image).
    height: Option<usize>,
    /// Width of the file (if image).
    width: Option<usize>,
}
