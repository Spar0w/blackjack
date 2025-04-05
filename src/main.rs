//blackjack!
use std::io;
use rand::Rng;

struct Card {
    suit: char,
    value: u32,
    symbol: String,
}

struct Player {
    hand: Vec<Card>,
    playing: bool,
}


fn deck_init() -> Vec<Card> {
    // function to initialize deck
    //
    // we'll first build the deck as a vector for practice, but this should be a stack

    let mut deck: Vec<Card> = Vec::new();

    //generate the deck
    for su in &['♠', '♥', '♦', '♣'] {
        //add numbered cards
        for x in 2..=10 {
            deck.push(Card {suit: *su, value: x, symbol: x.to_string()});
        }
        //face cards are just 10
        for face in ["J", "Q", "K"]{
            deck.push(Card {suit: *su, value: 10, symbol: face.to_string()});
        }
        //add the ace
        deck.push(Card {suit: *su, value: 1, symbol: "A".to_string()})
    }
    return deck;
}

fn deal(deck: &mut Vec<Card>) -> Card {
    //takes in a deck reference, removes a random card and returns it's value
    let avail_cards = deck.len()-1;

    if avail_cards > 0 {
        let draw = rand::thread_rng().gen_range(1..=avail_cards);
        println!("Delt a {} of {}", deck[draw].symbol, deck[draw].suit );
        return deck.swap_remove(draw);
    } else {
        //return a joker. will make better error handling later
        return Card {suit: 'J', value: 0, symbol: "J".to_string()};
    }
}

fn score(hand: &Vec<Card>) -> u32 {
    //add all card values in hand
    let mut sum = 0;
    for card in hand {
        if card.symbol == "A" && !(sum + 11 > 21){
            sum += 11;
        } else {
            sum += card.value;
        }
    }
    return sum;
}

fn show_hand(hand: &Vec<Card>) {
    //formats hand to the screen
    for card in hand {
        println!("{} of {}", card.symbol, card.suit);
    }
}
fn show_deck(deck: &Vec<Card>) {
    //formats deck to the screen
    for card in deck {
        println!("{} of {}", card.symbol, card.suit);
    }
}

fn check_bust(hand: &Vec<Card>) -> bool {
    //check if the hand has busted
    if score(hand) > 21 {
        //println!("Hand exceeds 21.");
        return true;
    } else {
        return false;
    }
}

fn blackjack(hand: &Vec<Card>) -> bool {
    //if player has 21, return true
    if score(hand) == 21 {
        return true;
    }
    return false;
}

fn results(player: &Player, dealer: &Player) {
    //determine the winner
    if ((score(&player.hand) > score(&dealer.hand)) && !check_bust(&player.hand)) || check_bust(&dealer.hand)   {
        println!("Player wins!");
    } else {
        println!("Dealer wins!");
    }
    println!("Final Score: ");
    println!("Player: {}", score(&player.hand));
    println!("Dealer: {}", score(&dealer.hand));
    return
}

fn game_loop(deck: &mut Vec<Card>, player: &mut Player, dealer: &mut Player) {
    //play as long as this is true
    let mut not_bust = true;

    //deal initial cards to player and dealer
    player.hand.push(deal(deck));
    player.hand.push(deal(deck));

    dealer.hand.push(deal(deck));
    dealer.hand.push(deal(deck));

    //as long as both are playing and no one has busted
    while not_bust && (player.playing || dealer.playing) {
    // the game loop
        println!("Cards in hand:");
        show_hand(&player.hand);
        println! ("Your Score: {}", score(&player.hand));
        println!();
        println!("Cards in dealer's hand:");
        show_hand(&dealer.hand);
        println! ("Dealer Score: {}", score(&dealer.hand));
        println!();

        //check blackjack
        if (blackjack(&player.hand)) { player.playing=false };
        if (blackjack(&dealer.hand)) { dealer.playing=false };

        //player turn
        if (player.playing){
            //hit or stand
            let mut response = String::new();

            println!("(h)it or (s)tand");

            io::stdin()
                .read_line(&mut response)
                .expect("Failed to read line");

            match response.trim_end() {
                "h" => { player.hand.push(deal(deck)); if check_bust(&player.hand) { not_bust=false; } },
                "s" => { player.playing = false; },
                _ => println!("try again"),
            }
        }
        
        //dealer turn
        if (not_bust && score(&player.hand) > score(&dealer.hand)) && dealer.playing {
            dealer.hand.push(deal(deck));
            if check_bust(&dealer.hand) { not_bust=false; }
        } else {
            //stand
            dealer.playing = false;
        }
    }
    //determine winner
    results(&player, &dealer);
}

fn main() {
    let mut deck = deck_init();
    let mut player_hand: Vec<Card> = Vec::new();
    let mut dealer_hand: Vec<Card> = Vec::new();

    let mut player = Player { hand: player_hand, playing: true };
    let mut dealer = Player { hand: dealer_hand, playing: true };
    
    game_loop(&mut deck, &mut player, &mut dealer);

    //println!("{}", deck.len());
}
