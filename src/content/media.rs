#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn get_description_by_first_approach(&self) -> String {
        // if we have a book:
        if let Media::Book { title, author } = self {
            format!("Book: {} written by {}", title, author)
        }
        // if we have a movie:
        else if let Media::Movie { title, director } = self {
            format!("Movie: {} directed by {}", title, director)
        }
        // if we have a audiobook:
        else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media description")
        }
    }

    pub fn get_description_by_better_approach(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} written by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} directed by {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}
