#[derive(Debug)]

enum Media{
    Book{title: String, author: String, pages: u32},
    Movie{title: String, director: String, duration: u32},
    Audiobook{title: String, author: String, duration: u32},
}

fn print_media(media: Media) {
    println!("{:?}", media);
}
fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        duration: 600,
    };

    print_media(audiobook);
}