use crate::mpris::generated::player::Metadata;

#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub image: Option<String>,
    pub position: i64,
    pub duration: i64,
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.artist == other.artist
            && self.title == other.title
            && self.album == other.album
            && self.image == other.image
            && self.duration == other.duration
    }
}

impl Track {
    #[must_use]
    pub fn new(metadata: Metadata, position: i64) -> Option<Self> {
        let artist = metadata
            .artists
            .map_or("Unknown artist".into(), |artists| artists.join(", "));
        let title = metadata.title.unwrap_or_else(|| "Unknown title".into());
        let album = metadata.album.unwrap_or_else(|| "Unknown album".into());
        let duration = metadata.length?;

        let image = metadata
            .art_url
            .and_then(|url| url.strip_prefix("file://").map(ToString::to_string));

        Some(Self {
            id: metadata.track_id.unwrap_or_else(|| "UNKNOWN".into()),
            artist,
            title,
            album,
            image,
            position,
            duration,
        })
    }
}
