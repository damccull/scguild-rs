use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Embed {
    /// Title of the embed.
    title: Option<String>,
    /// Type of embed (always "rich" for webhook embeds).
    r#type: Option<String>,
    ///description of embed.
    description: Option<String>,
    ///url of embed.
    url: Option<String>,
    /// Timestamp of embed content in ISO8601 format.
    timestamp: Option<String>,
    /// Color code of the embed.
    color: Option<usize>,
    /// Footer information.
    footer: Option<EmbedFooter>,
    /// Image information.
    image: Option<EmbedImage>,
    /// Thumbnail information.
    thumbnail: Option<EmbedThumbnail>,
    /// Video information.
    video: Option<EmbedVideo>,
    /// Provider information.
    provider: Option<EmbedProvider>,
    /// Author information.
    author: Option<EmbedAuthor>,
    /// Fields information.
    fields: Option<Vec<EmbedField>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    /// Source URL of the thumbnail. Supports only http(s) and attachments.
    url: Option<String>,
    /// A proxied URL of the thumbnail.
    proxy_url: Option<String>,
    /// Height of the thumbnail.
    height: Option<usize>,
    /// Width of the thumbnail.
    width: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedVideo {
    /// Source URL of the video. Supports only http(s) and attachments.
    url: Option<String>,
    /// A proxied URL of the video.
    proxy_url: Option<String>,
    /// Height of the video.
    height: Option<usize>,
    /// Width of the video.
    width: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedImage {
    /// Source URL of the image. Supports only http(s) and attachments.
    url: Option<String>,
    /// A proxied URL of the image.
    proxy_url: Option<String>,
    /// Height of the image.
    height: Option<usize>,
    /// Width of the image.
    width: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedProvider {
    /// Name of the provider.
    name: Option<String>,
    /// URL of the provider.
    url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedAuthor {
    /// Name of the author.
    name: Option<String>,
    /// URL of the author.
    url: Option<String>,
    /// URL of the author icon. Supports only http(s) and attachments.
    icon_url: Option<String>,
    /// A proxied url of the author icon.
    proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedFooter {
    /// The footer text.
    text: String,
    /// URL of footer icon. Only supports http(s) and attachments.
    icon_url: Option<String>,
    /// A proxied url of footer icon.
    proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbedField {
    /// Name of the field.
    name: String,
    /// Value of the field.
    value: String,
    /// Whether or not this field should display inline..
    inline: Option<bool>,
}

#[deprecated]
pub enum EmbedType {
    Rich,
    Image,
    Video,
    Gifv,
    Article,
    Link,
}
