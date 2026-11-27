#[derive(Debug)]

enum Media{
    Book{title: String, author: String, pages: u32},
    Movie{title: String, director: String, duration: u32},
    Audiobook{title: String, author: String, duration: u32},
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author, pages } => {
                format!("Book: '{}' by {} ({} pages)", title, author, pages)
            }
            Media::Movie { title, director, duration } => {
                format!("Movie: '{}' directed by {} ({} minutes)", title, director, duration)
            }
            Media::Audiobook { title, author, duration } => {
                format!("Audiobook: '{}' by {} ({} minutes)", title, author, duration)
            }
        }
    } 
}

struct Catalog {
    items: Vec<Media>,
}

/* Catalog is a collection of media items */
 impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add_item(&mut self, item: Media) {
        self.items.push(item);
    }

    fn list_items(&self) {
        for item in &self.items {
            println!("{}", item.description());
        }
    }
}

// // fn print_media(media: Media) {
//     println!("{:?} \n", media);
// }

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        duration: 600,
    };

    let book = Media::Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        pages: 450,
    };

    let movie = Media::Movie {
        title: String::from("The Matrix"),
        director: String::from("The Wachowskis"),
        duration: 136,
    };

    let mut catalog = Catalog::new();
    catalog.add_item(audiobook);
    catalog.add_item(book);
    catalog.add_item(movie);

    catalog.list_items();

    
    // println!("{} \n", audiobook.description());
    // println!("{} \n", book.description());
    // println!("{} \n", movie.description());

    // print_media(audiobook);
    // print_media(book);
    // print_media(movie);

}