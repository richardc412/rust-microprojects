// Exercise: Implement describe, matching on Rank first, then Suit.

// Stretch: Generate a full 52-card Vec<Card> and print them.

#[derive(Debug)]
enum Suit { Clubs, Diamonds, Hearts, Spades }
enum Rank {
    Num(u8),  // 2–10
    Jack,
    Queen,
    King,
    Ace,
}

struct Card(Rank, Suit);

fn describe(card: &Card) -> String {
    // match on card.0 and card.1
  let Card(rank, suit)= card;
    
  let rank_str: &str = match rank {
    Rank::Num(n) => &n.to_string(),
    Rank::Jack => "Jack",
    Rank::Queen => "Queen",
    Rank::King => "King",
    Rank::Ace => "Ace",
  };

  let suit_str: &str = match suit {
    Suit::Clubs => "Clubs",
    Suit::Diamonds => "Diamonds",
    Suit::Hearts => "Hearts",
    Suit::Spades => "Spades",
  };
  rank_str.to_string() + " of " + suit_str
}

pub fn run() {
    // Queen of Hearts (your original example)
    let card = Card(Rank::Queen, Suit::Hearts);
    assert_eq!(describe(&card), "Queen of Hearts");

    // Number cards
    let two_of_clubs    = Card(Rank::Num(2), Suit::Clubs);
    let ten_of_spades   = Card(Rank::Num(10), Suit::Spades);
    assert_eq!(describe(&two_of_clubs),  "2 of Clubs");
    assert_eq!(describe(&ten_of_spades), "10 of Spades");

    // Face cards in different suits
    let jack_of_diamonds  = Card(Rank::Jack, Suit::Diamonds);
    let king_of_clubs     = Card(Rank::King, Suit::Clubs);
    assert_eq!(describe(&jack_of_diamonds), "Jack of Diamonds");
    assert_eq!(describe(&king_of_clubs),    "King of Clubs");

    // Aces
    let ace_of_spades   = Card(Rank::Ace, Suit::Spades);
    let ace_of_diamonds = Card(Rank::Ace, Suit::Diamonds);
    assert_eq!(describe(&ace_of_spades),   "Ace of Spades");
    assert_eq!(describe(&ace_of_diamonds), "Ace of Diamonds");

    // Mixed random examples
    let seven_of_hearts = Card(Rank::Num(7), Suit::Hearts);
    let queen_of_spades = Card(Rank::Queen, Suit::Spades);
    assert_eq!(describe(&seven_of_hearts), "7 of Hearts");
    assert_eq!(describe(&queen_of_spades), "Queen of Spades");

    println!("All card descriptions are correct!");
}