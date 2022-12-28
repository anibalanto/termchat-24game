use indexmap::{indexmap, IndexMap};
use super::common::{Card, CardType};
use lazy_static::lazy_static;
use crate::cardascii::game::CardStack;


fn draw_card(card_visual: &Vec<&'static str>) -> String {
    let mut res = "".to_string();
    for str in card_visual {
        res.push_str(str);
        res.push_str("\n");
    }
    res
}

fn get_visual_card_from_vec_cards(hand: & Vec<&Card>, index: usize) -> String {
    draw_card(& CARD_STR_FRONTS.get( *( hand.get(index).unwrap() ) ).unwrap() )
}

fn get_visual_card_from_stack(hand: &CardStack, index: usize) -> String {
    draw_card(& CARD_STR_FRONTS.get_index( usize::from( * (hand.card_ids.get(index).unwrap() ) ) ).unwrap().1 )
}

fn get_visual_card_from_array(hand: & [Card; 4], index: usize) -> String {
    draw_card(& CARD_STR_FRONTS.get( & hand[index] ).unwrap() )
}


fn draw_hand<F>(get_visual_card: F) -> Vec<Vec<String>>
    where
        F: for<'a> Fn(&'a usize) -> String {
    vec![
        vec![get_visual_card(& 0usize),
             get_visual_card(& 1usize)],
        vec![get_visual_card(& 2usize),
             get_visual_card(& 3usize)],
    ]
}


pub fn draw_hand_from_vec_cards(hand: & Vec<&Card>) -> Vec<Vec<String>> {
    draw_hand( |indx: & usize| get_visual_card_from_vec_cards(hand, * indx) )
}


pub fn draw_hand_from_stack(hand: &CardStack) -> Vec<Vec<String>> {
    draw_hand( |indx: & usize| get_visual_card_from_stack(hand,* indx) )
}


pub fn draw_hand_from_array(hand: & [Card; 4]) -> Vec<Vec<String>> {
    draw_hand( |indx: & usize| get_visual_card_from_array(hand, * indx) )

}


macro_rules! make_str_card {
    ( $( $x:expr ),* ) => {
        {
            let mut vec = Vec::<&'static str>::new();
            $(
                #[allow(unused_assignments)]
                {
                    vec.push($x);
                }
            )*
            vec
        }
    };
}

/*fn card_str_back() -> Vec<&'static str>{
    make_str_card!(
        r#"┌────────────┐"#,
        r#"│╳╳╳╳╳╳╳╳╳╳╳╳│"#,
        r#"│╳╳╳╳╳╳╳╳╳╳╳╳│"#,
        r#"│╳╳╳╳╳╳╳╳╳╳╳╳│"#,
        r#"│╳CARDASCII!╳│"#,
        r#"│╳╳╳╳╳╳╳╳╳╳╳╳│"#,
        r#"│╳╳╳╳╳╳╳╳╳╳╳╳│"#,
        r#"│╳╳╳╳╳╳╳╳╳╳╳╳│"#,
        r#"└────────────┘"#)
}*/

pub fn make_card( _type: CardType, value: u8) -> Card {
    Card{ _type, value }
}

lazy_static! {
    static ref CARD_STR_FRONTS: IndexMap<Card, Vec<&'static str>> = {
        let map = indexmap! {
            make_card( CardType::Joker, 0 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│J    ◔   ⊙  │"#,
                r#"│O  ๏ |\  |\ │"#,
                r#"│K  |\/ |/ | │"#,
                r#"│E  ʕ  ͡o  ͡o| │"#,
                r#"│R  °༽   ͜ʖ༼  │"#,
                r#"│     ༽  ༼   │"#,
                r#"│            │"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Joker, 0 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│J    ◔   ⊙  │"#,
                r#"│O  ๏ |\  |\ │"#,
                r#"│K  |\/ |/ | │"#,
                r#"│E  ʕ  ͡o  ͡o| │"#,
                r#"│R  °༽   ͜ʖ༼  │"#,
                r#"│     ༽  ༼   │"#,
                r#"│            │"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Sword, 12 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│12  /^^^┼^\ │"#,
                r#"│|\ (  ° ͜ʖ° )│"#,
                r#"│ \\ \     / │"#,
                r#"│ _\\_---⊙-\ │"#,
                r#"│   ฿   .๏. \│"#,
                r#"│  /    .๏.  │"#,
                r#"│ /     .๏.12│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 11 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│11    ┌──@─┐│"#,
                r#"│|\    (° ͜ʖ°)│"#,
                r#"│ \\   /    \│"#,
                r#"│ _\\_Λ  Λ   │"#,
                r#"│   ฿(⊙  ⊙)\~│"#,
                r#"│     )  (  \│"#,
                r#"│     (..) 11│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 10 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│10   ┌───@┐ │"#,
                r#"│     │____│ │"#,
                r#"│  |\ (° ͜ʖ°) │"#,
                r#"│   \\/    \ │"#,
                r#"│   _\\_   / │"#,
                r#"│     ฿\  /฿ │"#,
                r#"│       || 10│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 9 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│9           │"#,
                r#"│            │"#,
                r#"│   |\       │"#,
                r#"│    \\      │"#,
                r#"│    _\\_    │"#,
                r#"│      \     │"#,
                r#"│           9│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 8 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│8           │"#,
                r#"│            │"#,
                r#"│   |\       │"#,
                r#"│    \\      │"#,
                r#"│    _\\_    │"#,
                r#"│      \     │"#,
                r#"│           8│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 7 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│7           │"#,
                r#"│            │"#,
                r#"│   |\       │"#,
                r#"│    \\      │"#,
                r#"│    _\\_    │"#,
                r#"│      \     │"#,
                r#"│           7│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 6 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│6           │"#,
                r#"│            │"#,
                r#"│   |\       │"#,
                r#"│    \\      │"#,
                r#"│    _\\_    │"#,
                r#"│      \     │"#,
                r#"│           6│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 5 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│5           │"#,
                r#"│            │"#,
                r#"│   |\       │"#,
                r#"│    \\      │"#,
                r#"│    _\\_    │"#,
                r#"│      \     │"#,
                r#"│           5│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 4 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│4           │"#,
                r#"│            │"#,
                r#"│   |\       │"#,
                r#"│    \\      │"#,
                r#"│    _\\_    │"#,
                r#"│      \     │"#,
                r#"│           4│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 3 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│3           │"#,
                r#"│            │"#,
                r#"│   |\       │"#,
                r#"│    \\      │"#,
                r#"│    _\\_    │"#,
                r#"│      \     │"#,
                r#"│           3│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 2 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│2           │"#,
                r#"│            │"#,
                r#"│   |\       │"#,
                r#"│    \\      │"#,
                r#"│    _\\_    │"#,
                r#"│      \     │"#,
                r#"│           2│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Sword, 1 ) => make_str_card!(
                r#"┌──  ────  ──┐"#,
                r#"│1           │"#,
                r#"│            │"#,
                r#"│   |\       │"#,
                r#"│    \\      │"#,
                r#"│    _\\_    │"#,
                r#"│      \     │"#,
                r#"│           1│"#,
                r#"└──  ────  ──┘"#)
            ,
            make_card( CardType::Club, 12 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│12  /^^^┼^\ │"#,
                r#"│.-.(  ° ͜ʖ° )│"#,
                r#"│(  )\     / │"#,
                r#"│ ( )/---⊙-\ │"#,
                r#"│  ()   .๏. \│"#,
                r#"│  /    .๏.  │"#,
                r#"│ /     .๏.12│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 11 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│11    ┌──@─┐│"#,
                r#"│.-.   (° ͜ʖ°)│"#,
                r#"│(  )  /    \│"#,
                r#"│ ( ) Λ  Λ   │"#,
                r#"│  ()(⊙  ⊙)\~│"#,
                r#"│     )  (  \│"#,
                r#"│     (..) 11│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 10 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│10   ┌───@┐ │"#,
                r#"│.-.  │____│ │"#,
                r#"│(  ) (° ͜ʖ°) │"#,
                r#"│ ( ) /    \ │"#,
                r#"│  ฿)/\    / │"#,
                r#"│      \  /฿ │"#,
                r#"│       || 10│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 9 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│9           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│    (  )    │"#,
                r#"│     ( )    │"#,
                r#"│      ()    │"#,
                r#"│           9│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 8 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│8           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│    (  )    │"#,
                r#"│     ( )    │"#,
                r#"│      ()    │"#,
                r#"│           8│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 7 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│7           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│    (  )    │"#,
                r#"│     ( )    │"#,
                r#"│      ()    │"#,
                r#"│           7│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 6 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│6           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│    (  )    │"#,
                r#"│     ( )    │"#,
                r#"│      ()    │"#,
                r#"│           6│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 5 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│5           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│    (  )    │"#,
                r#"│     ( )    │"#,
                r#"│      ()    │"#,
                r#"│           5│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 4 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│4           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│    (  )    │"#,
                r#"│     ( )    │"#,
                r#"│      ()    │"#,
                r#"│           4│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 3 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│3           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│    (  )    │"#,
                r#"│     ( )    │"#,
                r#"│      ()    │"#,
                r#"│           3│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 2 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│2           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│    (  )    │"#,
                r#"│     ( )    │"#,
                r#"│      ()    │"#,
                r#"│           2│"#,
                r#"└─  ──  ──  ─┘"#)
            ,
            make_card( CardType::Club, 1 ) => make_str_card!(
                r#"┌─  ──  ──  ─┐"#,
                r#"│1           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│    (  )    │"#,
                r#"│     ( )    │"#,
                r#"│      ()    │"#,
                r#"│           1│"#,
                r#"└─  ──  ──  ─┘"#)
            ,

            make_card( CardType::Gold, 12 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│12  /^^^┼^\ │"#,
                r#"│   (  ° ͜ʖ° )│"#,
                r#"│ .-.\     / │"#,
                r#"│( O )---⊙-\ │"#,
                r#"│ `฿`   .๏. \│"#,
                r#"│  /    .๏.  │"#,
                r#"│ /     .๏.12│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 11 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│11    ┌──@─┐│"#,
                r#"│ .-.  (° ͜ʖ°)│"#,
                r#"│( O ) /    \│"#,
                r#"│ `-฿ Λ  Λ   │"#,
                r#"│    (⊙  ⊙)\~│"#,
                r#"│     )  (  \│"#,
                r#"│     (..) 11│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 10 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│10   ┌───@┐ │"#,
                r#"│     │____│ │"#,
                r#"│ .-. (° ͜ʖ°) │"#,
                r#"│( O )/    \ │"#,
                r#"│ `฿` \    / │"#,
                r#"│      \  /฿ │"#,
                r#"│       || 10│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 9 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│9           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│   ( O )    │"#,
                r#"│    `-`     │"#,
                r#"│            │"#,
                r#"│           9│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 8 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│8           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│   ( O )    │"#,
                r#"│    `-`     │"#,
                r#"│            │"#,
                r#"│           8│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 7 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│7           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│   ( O )    │"#,
                r#"│    `-`     │"#,
                r#"│            │"#,
                r#"│           7│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 6 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│6           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│   ( O )    │"#,
                r#"│    `-`     │"#,
                r#"│            │"#,
                r#"│           6│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 5 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│5           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│   ( O )    │"#,
                r#"│    `-`     │"#,
                r#"│            │"#,
                r#"│           5│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 4 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│4           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│   ( O )    │"#,
                r#"│    `-`     │"#,
                r#"│            │"#,
                r#"│           4│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 3 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│3           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│   ( O )    │"#,
                r#"│    `-`     │"#,
                r#"│            │"#,
                r#"│           3│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 2 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│2           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│   ( O )    │"#,
                r#"│    `-`     │"#,
                r#"│            │"#,
                r#"│           2│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Gold, 1 ) => make_str_card!(
                r#"┌────────────┐"#,
                r#"│1           │"#,
                r#"│            │"#,
                r#"│    .-.     │"#,
                r#"│   ( O )    │"#,
                r#"│    `-`     │"#,
                r#"│            │"#,
                r#"│           1│"#,
                r#"└────────────┘"#)
            ,
            make_card( CardType::Cup, 12 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│12  /^^^┼^\ │"#,
                r#"│   (  ° ͜ʖ° )│"#,
                r#"│ ___\     / │"#,
                r#"│(___)---⊙-\ │"#,
                r#"│ ฿_/   .๏. \│"#,
                r#"│  /    .๏.  │"#,
                r#"│ /     .๏.12│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 11 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│11    ┌──@─┐│"#,
                r#"│ ___  (° ͜ʖ°)│"#,
                r#"│(___) /    \│"#,
                r#"│ \_฿ Λ  Λ   │"#,
                r#"│    (⊙  ⊙)\~│"#,
                r#"│     )  (  \│"#,
                r#"│     (..) 11│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 10 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│10   ┌───@┐ │"#,
                r#"│     │____│ │"#,
                r#"│ ___ (° ͜ʖ°) │"#,
                r#"│(___)/    \ │"#,
                r#"│ \_฿ \    / │"#,
                r#"│      \  /฿ │"#,
                r#"│       || 10│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 9 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│9           │"#,
                r#"│            │"#,
                r#"│    ___     │"#,
                r#"│   (___)    │"#,
                r#"│    \_/     │"#,
                r#"│            │"#,
                r#"│           9│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 8 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│8           │"#,
                r#"│            │"#,
                r#"│    ___     │"#,
                r#"│   (___)    │"#,
                r#"│    \_/     │"#,
                r#"│            │"#,
                r#"│           8│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 7 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│7           │"#,
                r#"│            │"#,
                r#"│    ___     │"#,
                r#"│   (___)    │"#,
                r#"│    \_/     │"#,
                r#"│            │"#,
                r#"│           7│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 6 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│6           │"#,
                r#"│            │"#,
                r#"│    ___     │"#,
                r#"│   (___)    │"#,
                r#"│    \_/     │"#,
                r#"│            │"#,
                r#"│           6│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 5 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│5           │"#,
                r#"│            │"#,
                r#"│    ___     │"#,
                r#"│   (___)    │"#,
                r#"│    \_/     │"#,
                r#"│            │"#,
                r#"│           5│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 4 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│4           │"#,
                r#"│            │"#,
                r#"│    ___     │"#,
                r#"│   (___)    │"#,
                r#"│    \_/     │"#,
                r#"│            │"#,
                r#"│           4│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 3 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│3           │"#,
                r#"│            │"#,
                r#"│    ___     │"#,
                r#"│   (___)    │"#,
                r#"│    \_/     │"#,
                r#"│            │"#,
                r#"│           3│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 2 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│2           │"#,
                r#"│            │"#,
                r#"│    ___     │"#,
                r#"│   (___)    │"#,
                r#"│    \_/     │"#,
                r#"│            │"#,
                r#"│           2│"#,
                r#"└────    ────┘"#)
            ,
            make_card( CardType::Cup, 1 ) => make_str_card!(
                r#"┌────    ────┐"#,
                r#"│1           │"#,
                r#"│            │"#,
                r#"│    ___     │"#,
                r#"│   (___)    │"#,
                r#"│    \_/     │"#,
                r#"│            │"#,
                r#"│           1│"#,
                r#"└────    ────┘"#)
        };
        map

    };
}
