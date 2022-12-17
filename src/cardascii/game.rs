use rand::prelude::SliceRandom;
use rand::thread_rng;
use rcalc::{Interpreter, Lexer, Token};
use super::{common::{Card, CardType}};
use bimap::BiMap;
use std::collections::HashMap;
/*const CARD_ID_JOCKER_1: u8 = 0;
const CARD_ID_JOCKER_2: u8 = 1;
*/

#[derive(Debug)]
pub struct Game24Err(pub String);

struct Deck{
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Self {

        let mut me = Deck {
            cards : Vec::<Card>::new(),
        };

        load_cards(& mut me);

        me
    }

    fn add(&mut self, _type: CardType, value: u8) {
        self.cards.push( Card{ _type, value } );
    }

    /*fn as_ids(& self) -> Vec<u8> {
        (0 .. self.cards.len() as u8).collect()
    }*/

    fn as_ids_no_jokers(& self) -> Vec<u8> {
        (0 .. self.cards.len() as u8).collect()
    }

    fn get_card(& self, id: & u8) -> Option<&Card> {
        self.cards.get(*id as usize)
    }

    fn get_card_pos(& self, i: usize, card_ids : & Vec<u8>) -> Option<&Card> {
        match card_ids.get(i) {
            Some(id)    => self.get_card(id),
            None            => None
        }
    }

    fn get_cards_from_stack(& self, stack: & CardStack) -> Vec<&Card> {
        stack.card_ids
            .iter()
            .map( |card_id| self.get_card(card_id) )
            .filter(|opt_card| opt_card.is_some() )
            .map(|opt_card| opt_card.unwrap())
            .collect()
    }

}

pub struct CardStack {
    is_face_up:bool,
    pub card_ids: Vec<u8>,
}

impl CardStack {
    fn new(is_face_up: bool) -> Self {
        CardStack {
            is_face_up,
            card_ids: Vec::<u8>::new()
        }
    }

    fn add_cards(&mut self, deck: &Deck) {
        self.card_ids = deck.as_ids_no_jokers();
    }

    fn add_all_from(&mut self, from: &mut CardStack) {
        self.card_ids.append(& mut from.card_ids);
        //from.card_ids.clear();
    }

    fn add_one_from(&mut self, from: &mut CardStack) -> bool {
        let mut result = false;
        if let Some(id) = from.card_ids.pop() {
            self.card_ids.push(id);
            result = true;
        }
        result
    }

    fn add_n_from(&mut self, from: &mut CardStack, n: u8) -> bool {
        let mut result = true;
        for _ in 0..n {
            if !self.add_one_from(from) {
                result = false;
                break
            }
        }
        result
    }

    fn shuffle(&mut self) {
        self.card_ids.shuffle(&mut thread_rng());
    }

    fn is_empty(&self) -> bool {
        self.card_ids.is_empty()
    }

}


#[derive(PartialEq)]
pub enum TurnResult {
    Winner(usize),
    Tie,
    Gaming,
    Abandoned
}

pub struct Turn {
    num:            u8,
    pub visible_cards:  CardStack,
    pub result:         TurnResult
}

pub struct Game24{
    player_count:       usize,
    players:            BiMap<String, usize>,
    players_gaming_turn:HashMap<usize, bool>,
    deck:               Deck,
    hidden_cards:       CardStack,
    players_cards:      Vec<CardStack>,
    accumulate_cards:   CardStack,
    turn:               Turn
}

impl Game24 {
    pub(crate) fn new(/*player: u8, deck: &'a Deck*/) -> Self {
        let mut hidden_cards = CardStack::new(false);
        let deck = Deck::new();
        hidden_cards.add_cards(&deck);
        hidden_cards.shuffle();
        let mut players_cards = Vec::<CardStack>::new();

        players_cards.push(CardStack::new(false));
        players_cards.push(CardStack::new(false));

        Game24 {
            player_count: 0,
            players: BiMap::new(),
            players_gaming_turn: HashMap::new(),
            deck,
            hidden_cards,
            players_cards,
            accumulate_cards: CardStack::new(false),
            turn: Turn {
                num: 0,
                visible_cards: CardStack::new(true),
                result: TurnResult::Gaming
            }
        }
    }

    pub fn do_reset(&mut self) {
        self.hidden_cards.add_all_from( &mut self.turn.visible_cards );
        for player_card in & mut self.players_cards.iter_mut() {
            self.hidden_cards.add_all_from( player_card );
        }
        
        for player in self.players.right_values().enumerate() {
            self.players_gaming_turn.insert(player.0, false);
        }

        self.hidden_cards.shuffle();
    }

    pub fn do_give_cards(&mut self) -> Result< & Turn , Game24Err>{
        if ! self.hidden_cards.is_empty() && self.turn.visible_cards.add_n_from(&mut self.hidden_cards, 4) {
            self.turn.num += 1;
            self.turn.result = TurnResult::Gaming;
            return Ok( & self.turn );
        }
        Err(Game24Err(format!("we can't do cards")))
    }

    pub fn get_gived_cards(&self) -> Vec<&Card> {
        self.deck.get_cards_from_stack(& self.turn.visible_cards)
    }

    pub fn do_user_registration(& mut self, user: & String) -> Result< (), Game24Err > {
        match self.players.get_by_left(user) {
            None => {
                self.player_count += 1;
                self.players.insert(user.clone(), self.player_count);
                Ok(())
            },
            Some(_) => Err(Game24Err(format!("the user already exists")))
        }
    }

    pub fn do_pass(&mut self, user: & String) -> Result< & Turn, Game24Err >{
        match self.players.get_by_left(user).cloned() {
            Some(user) =>  {
                match self.players_gaming_turn.get_mut( &user ) {
                    Some(gaming_turn) => {
                        *gaming_turn = false;
                        if ! self.players_gaming_turn.values().any( |&gaming| gaming ) {
                            self.end_turn(TurnResult::Tie);
                        }
                        Ok( & self.turn )
                    },
                    None => Err(Game24Err(format!("User game state not registered")))
                }
            },
            _ => Err(Game24Err(format!("User not registered")))
        }
    }

    pub fn make_answer(&mut self, user: & String, answer: String) -> Result< & Turn, Game24Err > {
        let result_24 = self.validate_24_result(user, & answer);
        match result_24 {
            Ok(()) => match self.validate_card_usage_answer(user, & answer) {
                Ok(()) => Ok(& self.turn),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }

    fn end_turn(&mut self, result: TurnResult) {
        //write!(stdout, "{}{}turn: {} (push 'r' for next turn)", termion::clear::All, termion::cursor::Goto(1, 1), self.turn_num).unwrap();
        match result {
            TurnResult::Winner(user) => {
                self.players_cards[user].add_all_from(&mut self.accumulate_cards);
                self.players_cards[user].add_all_from(&mut self.turn.visible_cards);
            }
            TurnResult::Tie =>
                self.accumulate_cards.add_all_from(&mut self.turn.visible_cards),
            _ => ()
        }
        self.turn.result = result;
    }

    fn validate_24_result(&mut self, user: & String, answer: & String) -> Result< (), Game24Err > {
        match self.players.get_by_left(user) {
            Some(_) =>  {
                let mut program = Interpreter::from(answer.as_str());

                match program.interpret() {
                    Ok(result) => { 
                        if result == 24.0 {
                            Ok(())
                        } else {
                            Err(Game24Err(format!("the result of your operation isn't 24 result is {result}")))
                        }
                    },
                    Err(e) =>
                        Err(Game24Err(format!("the operation isn't correct\n{e}")))
                }
            },
            None => Err(Game24Err(format!("User not registered")))
        }
    }

    fn validate_card_usage_answer(&mut self, user: & String, answer: & String) -> Result< (), Game24Err > {
        let mut lexer = Lexer::from(answer.as_str());

        let mut cards_vec   = self.deck.get_cards_from_stack(& mut self.turn.visible_cards);

        while let Ok(token) =  lexer.next_token() {
            if token == Token::EOF {
                break;
            }
            if let Token::NUMBER(n)  = token {
                let pos = cards_vec
                    .iter()
                    .position( |x| usize::from(x.value) == n );

                if let Some(pos) = pos {
                    cards_vec.remove(pos);
                } else {
                    return Err(Game24Err(format!("you are using numbers that don't exist in this cards {n}")));
                }
            }
        }
        if cards_vec.is_empty() {
            match self.players.get_by_left(user).cloned() {
                Some(user) =>  {
                    self.end_turn(TurnResult::Winner(user));
                    Ok(())
                },
                None => Err(Game24Err(format!("User not registered")))
            }

        } else {
            Err(Game24Err(format!("don't use this cards {cards_vec:?}")))
        }
        
    }

}

fn load_cards(deck: & mut Deck) {

    deck.add(CardType::Joker, 0);
    deck.add(CardType::Joker, 0);

    deck.add(CardType::Sword, 12);
    deck.add(CardType::Sword, 11);
    deck.add(CardType::Sword, 10);
    deck.add(CardType::Sword, 9);
    deck.add(CardType::Sword, 8);
    deck.add(CardType::Sword, 7);
    deck.add(CardType::Sword, 6);
    deck.add(CardType::Sword, 5);
    deck.add(CardType::Sword, 4);
    deck.add(CardType::Sword, 3);
    deck.add(CardType::Sword, 2);
    deck.add(CardType::Sword, 1);

    deck.add(CardType::Club, 12);
    deck.add(CardType::Club, 11);
    deck.add(CardType::Club, 10);
    deck.add(CardType::Club, 9);
    deck.add(CardType::Club, 8);
    deck.add(CardType::Club, 7);
    deck.add(CardType::Club, 6);
    deck.add(CardType::Club, 5);
    deck.add(CardType::Club, 4);
    deck.add(CardType::Club, 3);
    deck.add(CardType::Club, 2);
    deck.add(CardType::Club, 1);

    deck.add(CardType::Gold, 12);
    deck.add(CardType::Gold, 11);
    deck.add(CardType::Gold, 10);
    deck.add(CardType::Gold, 9);
    deck.add(CardType::Gold, 8);
    deck.add(CardType::Gold, 7);
    deck.add(CardType::Gold, 6);
    deck.add(CardType::Gold, 5);
    deck.add(CardType::Gold, 4);
    deck.add(CardType::Gold, 3);
    deck.add(CardType::Gold, 2);
    deck.add(CardType::Gold, 1);

    deck.add(CardType::Cup, 12);
    deck.add(CardType::Cup, 11);
    deck.add(CardType::Cup, 10);
    deck.add(CardType::Cup, 9);
    deck.add(CardType::Cup, 8);
    deck.add(CardType::Cup, 7);
    deck.add(CardType::Cup, 6);
    deck.add(CardType::Cup, 5);
    deck.add(CardType::Cup, 4);
    deck.add(CardType::Cup, 3);
    deck.add(CardType::Cup, 2);
    deck.add(CardType::Cup, 1);

}
