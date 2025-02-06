use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media_item: Media) {
        self.items.push(media_item)
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // Good! We have something to return:
            Some(&self.items[index])
        } else {
            // Bad! The index is lager then our items vector:
            None
        }
    }
}
