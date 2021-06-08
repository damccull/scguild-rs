pub struct Embed {
    ///title of embed
    title: Option<String>,
    ///type of embed (always "rich" for webhook embeds)
    r#type: Option<String>,
    ///description of embed
    description: Option<String>,
    ///url of embed
    url: Option<String>,
    ///timestamp of embed content
    timestamp: Option<ISO8601timestamp>,
    ///color code of the embed
    color: Option<Color>,
    ///footer information
    footer: Option<EmbedFooter>,
    ///image information
    image: Option<EmbedImage>,
    ///thumbnail information
    thumbnail: Option<EmbedThumbnail>,
    ///video information
    video: Option<EmbedVideo>,
    ///provider information
    provider: Option<EmbedProvider>,
    ///author information
    author: Option<EmbedAuthor>,
    ///fields information
    fields: Option<Vec<EmbedField>>,
}
