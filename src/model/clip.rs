use crate::dal::response::clip_response::ClipResponse;

#[derive(Debug, Clone)]
pub struct Clip {
    pub id: String,
    pub title: String,
    pub caption: String,
    pub mp4_link: String,
    pub gif_link: String,
}

impl Clip {
    pub fn from(from: &ClipResponse) -> Clip {
        Clip {
            id: from.id.clone(),
            title: from.title.clone(),
            caption: from.caption.clone(),
            mp4_link: from.mp4_link.clone(),
            gif_link: from.gif_link.clone(),
        }
    }
}
