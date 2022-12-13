use std::collections::HashMap;
use super::common::{Card, CardType};
use lazy_static::lazy_static;

pub fn draw_hand(hand: Vec<&Card>) -> Vec<Vec<String>> {
    vec![
        vec![draw_card(& CARD_STR_FRONTS.get( & hand.get(0).unwrap() ).unwrap() ),
             draw_card(& CARD_STR_FRONTS.get( & hand.get(1).unwrap() ).unwrap() )],
        vec![draw_card(& CARD_STR_FRONTS.get( & hand.get(2).unwrap() ).unwrap() ),
             draw_card(& CARD_STR_FRONTS.get( & hand.get(3).unwrap() ).unwrap() )],
    ]
}

pub fn draw_hand_from_array(hand: & [Card; 4]) -> Vec<Vec<String>> {
    vec![
        vec![draw_card(& CARD_STR_FRONTS.get( & hand[0] ).unwrap() ),
             draw_card(& CARD_STR_FRONTS.get( & hand[1] ).unwrap() )],
        vec![draw_card(& CARD_STR_FRONTS.get( & hand[2] ).unwrap() ),
             draw_card(& CARD_STR_FRONTS.get( & hand[3] ).unwrap() )],
    ]
}
pub fn draw_card(card_visual: &Vec<&'static str>) -> String {
    let mut res = "".to_string();
    for str in card_visual {
        res.push_str(str);
        res.push_str("\n");
    }
    res
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

pub fn add(
        fronts: & mut HashMap<Card, Vec<&'static str>>, 
        _type: CardType, 
        value: u8, 
        visual: Vec<&'static str>) {

    fronts.insert( Card{ _type, value } , visual);

}

lazy_static! {
    static ref CARD_STR_FRONTS: HashMap<Card, Vec<&'static str>> = {
        let mut fronts = HashMap::new();
        add(& mut fronts, CardType::Joker, 0, make_str_card!(
            r#"┌────────────┐"#,
            r#"│J    ◔   ⊙  │"#,
            r#"│O  ๏ |\  |\ │"#,
            r#"│K  |\/ |/ | │"#,
            r#"│E  ʕ  ͡o  ͡o| │"#,
            r#"│R  °༽   ͜ʖ༼  │"#,
            r#"│     ༽  ༼   │"#,
            r#"│            │"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Joker, 0, make_str_card!(
            r#"┌────────────┐"#,
            r#"│J    ◔   ⊙  │"#,
            r#"│O  ๏ |\  |\ │"#,
            r#"│K  |\/ |/ | │"#,
            r#"│E  ʕ  ͡o  ͡o| │"#,
            r#"│R  °༽   ͜ʖ༼  │"#,
            r#"│     ༽  ༼   │"#,
            r#"│            │"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Sword, 12, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│12  /^^^┼^\ │"#,
            r#"│|\ (  ° ͜ʖ° )│"#,
            r#"│ \\ \     / │"#,
            r#"│ _\\_---⚙-\ │"#,
            r#"│   ฿   .๏. \│"#,
            r#"│  /    .๏.  │"#,
            r#"│ /     .๏.12│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 11, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│11    ┌──@─┐│"#,
            r#"│|\    (° ͜ʖ°)│"#,
            r#"│ \\   /    \│"#,
            r#"│ _\\_Λ  Λ   │"#,
            r#"│   ฿(⚙  ⚙)\~│"#,
            r#"│     )  (  \│"#,
            r#"│     (..) 11│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 10, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│10   ┌───@┐ │"#,
            r#"│     │____│ │"#,
            r#"│  |\ (° ͜ʖ°) │"#,
            r#"│   \\/    \ │"#,
            r#"│   _\\_   / │"#,
            r#"│     ฿\  /฿ │"#,
            r#"│       || 10│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 9, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│9           │"#,
            r#"│            │"#,
            r#"│   |\       │"#,
            r#"│    \\      │"#,
            r#"│    _\\_    │"#,
            r#"│      \     │"#,
            r#"│           9│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 8, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│8           │"#,
            r#"│            │"#,
            r#"│   |\       │"#,
            r#"│    \\      │"#,
            r#"│    _\\_    │"#,
            r#"│      \     │"#,
            r#"│           8│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 7, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│7           │"#,
            r#"│            │"#,
            r#"│   |\       │"#,
            r#"│    \\      │"#,
            r#"│    _\\_    │"#,
            r#"│      \     │"#,
            r#"│           7│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 6, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│6           │"#,
            r#"│            │"#,
            r#"│   |\       │"#,
            r#"│    \\      │"#,
            r#"│    _\\_    │"#,
            r#"│      \     │"#,
            r#"│           6│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 5, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│5           │"#,
            r#"│            │"#,
            r#"│   |\       │"#,
            r#"│    \\      │"#,
            r#"│    _\\_    │"#,
            r#"│      \     │"#,
            r#"│           5│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 4, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│4           │"#,
            r#"│            │"#,
            r#"│   |\       │"#,
            r#"│    \\      │"#,
            r#"│    _\\_    │"#,
            r#"│      \     │"#,
            r#"│           4│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 3, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│3           │"#,
            r#"│            │"#,
            r#"│   |\       │"#,
            r#"│    \\      │"#,
            r#"│    _\\_    │"#,
            r#"│      \     │"#,
            r#"│           3│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 2, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│2           │"#,
            r#"│            │"#,
            r#"│   |\       │"#,
            r#"│    \\      │"#,
            r#"│    _\\_    │"#,
            r#"│      \     │"#,
            r#"│           2│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Sword, 1, make_str_card!(
            r#"┌──  ────  ──┐"#,
            r#"│1           │"#,
            r#"│            │"#,
            r#"│   |\       │"#,
            r#"│    \\      │"#,
            r#"│    _\\_    │"#,
            r#"│      \     │"#,
            r#"│           1│"#,
            r#"└──  ────  ──┘"#)
        );
        add(& mut fronts, CardType::Club, 12, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│12  /^^^┼^\ │"#,
            r#"│.-.(  ° ͜ʖ° )│"#,
            r#"│(  )\     / │"#,
            r#"│ ( )/---⚙-\ │"#,
            r#"│  ()   .๏. \│"#,
            r#"│  /    .๏.  │"#,
            r#"│ /     .๏.12│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 11, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│11    ┌──@─┐│"#,
            r#"│.-.   (° ͜ʖ°)│"#,
            r#"│(  )  /    \│"#,
            r#"│ ( ) Λ  Λ   │"#,
            r#"│  ()(⚙  ⚙)\~│"#,
            r#"│     )  (  \│"#,
            r#"│     (..) 11│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 10, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│10   ┌───@┐ │"#,
            r#"│.-.  │____│ │"#,
            r#"│(  ) (° ͜ʖ°) │"#,
            r#"│ ( ) /    \ │"#,
            r#"│  ฿)/\    / │"#,
            r#"│      \  /฿ │"#,
            r#"│       || 10│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 9, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│9      .-.  │"#,
            r#"│       (  ) │"#,
            r#"│        ( ) │"#,
            r#"│         () │"#,
            r#"│            │"#,
            r#"│            │"#,
            r#"│           9│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 8, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│8           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│    (  )    │"#,
            r#"│     ( )    │"#,
            r#"│      ()    │"#,
            r#"│           8│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 7, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│7           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│    (  )    │"#,
            r#"│     ( )    │"#,
            r#"│      ()    │"#,
            r#"│           7│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 6, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│6           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│    (  )    │"#,
            r#"│     ( )    │"#,
            r#"│      ()    │"#,
            r#"│           6│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 5, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│5           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│    (  )    │"#,
            r#"│     ( )    │"#,
            r#"│      ()    │"#,
            r#"│           5│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 4, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│4           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│    (  )    │"#,
            r#"│     ( )    │"#,
            r#"│      ()    │"#,
            r#"│           4│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 3, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│3           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│    (  )    │"#,
            r#"│     ( )    │"#,
            r#"│      ()    │"#,
            r#"│           3│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 2, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│2           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│    (  )    │"#,
            r#"│     ( )    │"#,
            r#"│      ()    │"#,
            r#"│           2│"#,
            r#"└─  ──  ──  ─┘"#)
        );
        add(& mut fronts, CardType::Club, 1, make_str_card!(
            r#"┌─  ──  ──  ─┐"#,
            r#"│1           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│    (  )    │"#,
            r#"│     ( )    │"#,
            r#"│      ()    │"#,
            r#"│           1│"#,
            r#"└─  ──  ──  ─┘"#)
        );

        add(& mut fronts, CardType::Gold, 12, make_str_card!(
            r#"┌────────────┐"#,
            r#"│12  /^^^┼^\ │"#,
            r#"│   (  ° ͜ʖ° )│"#,
            r#"│ .-.\     / │"#,
            r#"│( O )---⚙-\ │"#,
            r#"│ `฿`   .๏. \│"#,
            r#"│  /    .๏.  │"#,
            r#"│ /     .๏.12│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 11, make_str_card!(
            r#"┌────────────┐"#,
            r#"│11    ┌──@─┐│"#,
            r#"│ .-.  (° ͜ʖ°)│"#,
            r#"│( O ) /    \│"#,
            r#"│ `-฿ Λ  Λ   │"#,
            r#"│    (⚙  ⚙)\~│"#,
            r#"│     )  (  \│"#,
            r#"│     (..) 11│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 10, make_str_card!(
            r#"┌────────────┐"#,
            r#"│10   ┌───@┐ │"#,
            r#"│     │____│ │"#,
            r#"│ .-. (° ͜ʖ°) │"#,
            r#"│( O )/    \ │"#,
            r#"│ `฿` \    / │"#,
            r#"│      \  /฿ │"#,
            r#"│       || 10│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 9, make_str_card!(
            r#"┌────────────┐"#,
            r#"│9           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│   ( O )    │"#,
            r#"│    `-`     │"#,
            r#"│            │"#,
            r#"│           9│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 8, make_str_card!(
            r#"┌────────────┐"#,
            r#"│8           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│   ( O )    │"#,
            r#"│    `-`     │"#,
            r#"│            │"#,
            r#"│           8│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 7, make_str_card!(
            r#"┌────────────┐"#,
            r#"│7           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│   ( O )    │"#,
            r#"│    `-`     │"#,
            r#"│            │"#,
            r#"│           7│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 6, make_str_card!(
            r#"┌────────────┐"#,
            r#"│6           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│   ( O )    │"#,
            r#"│    `-`     │"#,
            r#"│            │"#,
            r#"│           6│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 5, make_str_card!(
            r#"┌────────────┐"#,
            r#"│5           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│   ( O )    │"#,
            r#"│    `-`     │"#,
            r#"│            │"#,
            r#"│           5│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 4, make_str_card!(
            r#"┌────────────┐"#,
            r#"│4           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│   ( O )    │"#,
            r#"│    `-`     │"#,
            r#"│            │"#,
            r#"│           4│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 3, make_str_card!(
            r#"┌────────────┐"#,
            r#"│3           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│   ( O )    │"#,
            r#"│    `-`     │"#,
            r#"│            │"#,
            r#"│           3│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 2, make_str_card!(
            r#"┌────────────┐"#,
            r#"│2           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│   ( O )    │"#,
            r#"│    `-`     │"#,
            r#"│            │"#,
            r#"│           2│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Gold, 1, make_str_card!(
            r#"┌────────────┐"#,
            r#"│1           │"#,
            r#"│            │"#,
            r#"│    .-.     │"#,
            r#"│   ( O )    │"#,
            r#"│    `-`     │"#,
            r#"│            │"#,
            r#"│           1│"#,
            r#"└────────────┘"#)
        );
        add(& mut fronts, CardType::Cup, 12, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│12  /^^^┼^\ │"#,
            r#"│   (  ° ͜ʖ° )│"#,
            r#"│ ___\     / │"#,
            r#"│(___)---⚙-\ │"#,
            r#"│ ฿_/   .๏. \│"#,
            r#"│  /    .๏.  │"#,
            r#"│ /     .๏.12│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 11, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│11    ┌──@─┐│"#,
            r#"│ ___  (° ͜ʖ°)│"#,
            r#"│(___) /    \│"#,
            r#"│ \_฿ Λ  Λ   │"#,
            r#"│    (⚙  ⚙)\~│"#,
            r#"│     )  (  \│"#,
            r#"│     (..) 11│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 10, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│10   ┌───@┐ │"#,
            r#"│     │____│ │"#,
            r#"│ ___ (° ͜ʖ°) │"#,
            r#"│(___)/    \ │"#,
            r#"│ \_฿ \    / │"#,
            r#"│      \  /฿ │"#,
            r#"│       || 10│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 9, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│9           │"#,
            r#"│            │"#,
            r#"│    ___     │"#,
            r#"│   (___)    │"#,
            r#"│    \_/     │"#,
            r#"│            │"#,
            r#"│           9│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 8, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│8           │"#,
            r#"│            │"#,
            r#"│    ___     │"#,
            r#"│   (___)    │"#,
            r#"│    \_/     │"#,
            r#"│            │"#,
            r#"│           8│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 7, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│7           │"#,
            r#"│            │"#,
            r#"│    ___     │"#,
            r#"│   (___)    │"#,
            r#"│    \_/     │"#,
            r#"│            │"#,
            r#"│           7│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 6, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│6           │"#,
            r#"│            │"#,
            r#"│    ___     │"#,
            r#"│   (___)    │"#,
            r#"│    \_/     │"#,
            r#"│            │"#,
            r#"│           6│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 5, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│5           │"#,
            r#"│            │"#,
            r#"│    ___     │"#,
            r#"│   (___)    │"#,
            r#"│    \_/     │"#,
            r#"│            │"#,
            r#"│           5│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 4, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│4           │"#,
            r#"│            │"#,
            r#"│    ___     │"#,
            r#"│   (___)    │"#,
            r#"│    \_/     │"#,
            r#"│            │"#,
            r#"│           4│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 3, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│3           │"#,
            r#"│            │"#,
            r#"│    ___     │"#,
            r#"│   (___)    │"#,
            r#"│    \_/     │"#,
            r#"│            │"#,
            r#"│           3│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 2, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│2           │"#,
            r#"│            │"#,
            r#"│    ___     │"#,
            r#"│   (___)    │"#,
            r#"│    \_/     │"#,
            r#"│            │"#,
            r#"│           2│"#,
            r#"└────    ────┘"#)
        );
        add(& mut fronts, CardType::Cup, 1, make_str_card!(
            r#"┌────    ────┐"#,
            r#"│1           │"#,
            r#"│            │"#,
            r#"│    ___     │"#,
            r#"│   (___)    │"#,
            r#"│    \_/     │"#,
            r#"│            │"#,
            r#"│           1│"#,
            r#"└────    ────┘"#)
        );
        fronts
    };
}
