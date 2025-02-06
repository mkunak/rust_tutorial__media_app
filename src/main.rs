mod content;

use content::catalog::Catalog;
use content::media::Media;

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Test audiobook"),
    };

    let movie = Media::Movie {
        title: String::from("Interstellar"),
        director: String::from("Christopher Nolan"),
    };

    let book = Media::Book {
        title: String::from("Chipolino"),
        author: String::from("Janni Rodari"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    match catalog.items.get(10) {
        // Option::Some(value) => {
        //     println!("Item: {:#?}", value);
        // }
        // Option::None => {
        //     println!("Nothing at that index");
        // }
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing at that index");
        }
    }

    // println!("Catalog: {:#?}", catalog);
    // println!("Catalog: {:#?}", catalog.items.get(0));
    // println!("Catalog: {:#?}", catalog.items.get(100));

    // println!("{}", audiobook.get_description_by_first_approach());
    // println!("{}", movie.get_description_by_first_approach());
    // println!("{}", book.get_description_by_first_approach());

    // println!("{}", audiobook.get_description_by_better_approach());
    // println!("{}", movie.get_description_by_better_approach());
    // println!("{}", book.get_description_by_better_approach());

    // print_media(audiobook);
    // print_media(movie);
    // print_media(book);
}
