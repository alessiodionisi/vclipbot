use crate::dal::response::clip_response::ClipResponse;

pub struct Clip {
    pub id: String,
    pub title: String,
    pub caption: String,
}

impl Clip {
    pub fn from(from: &ClipResponse) -> Clip {
        Clip {
            id: from.id.clone(),
            title: from.title.clone(),
            caption: from.caption.clone(),
        }
    }
}