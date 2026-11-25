// 1. Define the blueprint for a Deck
struct Deck {
    cards_left: usize,
}

// 2. Define methods that belong to Deck
impl Deck {
    // Uses &self because we are only READING the number of cards
    fn check_count(&self) {
        println!("There are {} cards left.", self.cards_left);
    }

    // Uses &mut self because we are CHANGING the number of cards
    fn deal(&mut self, num_cards: usize) {
        self.cards_left -= num_cards;
    }
}

fn main() {
    // Create a specific instance of Deck
    let mut my_deck = Deck { cards_left: 52 };

    // Here, `self` inside the methods automatically refers to `my_deck`
    my_deck.check_count(); // Prints: There are 52 cards left.
    
    my_deck.deal(5);       // Removes 5 cards from `my_deck`
    
    my_deck.check_count(); // Prints: There are 47 cards left.
}