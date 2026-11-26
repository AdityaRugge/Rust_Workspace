#[derive(Debug)]

enum Media{
    Book{title: String, author: String, pages: u32},
    Movie{title: String, director: String, duration: u32},
    Audiobook{title: String, author: String, duration: u32},
}

fn print_media(media: Media) {
    println!("{:?} \n", media);
}
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

    print_media(audiobook);
    print_media(book);
    print_media(movie);
}