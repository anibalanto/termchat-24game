use std::collections::HashMap;
use super::common::{HandCardData, Card, CardType};

pub struct VisualDeck{
    //back:   Vec<&'static str>,
    fronts: HashMap<Card, Vec<&'static str>>
}

impl VisualDeck {
    pub fn new() -> Self {
        let mut me = VisualDeck {
            //back : card_str_back(),
            fronts : HashMap::<Card, Vec<&'static str>>::new()
        };

        load_cards_str_front(& mut me);

        me
    }

    pub fn add(&mut self, _type: CardType, value: u8, visual: Vec<&'static str>) {

        self.fronts.insert( Card{ _type, value } , visual);

    }

    pub fn draw_hand(& self, hand: Vec<&Card>) -> Vec<Vec<String>> {
        vec![
            vec![draw_card(& self.fronts.get( & hand.get(0).unwrap() ).unwrap() ),
                 draw_card(& self.fronts.get( & hand.get(1).unwrap() ).unwrap() )],
            vec![draw_card(& self.fronts.get( & hand.get(2).unwrap() ).unwrap() ),
                 draw_card(& self.fronts.get( & hand.get(3).unwrap() ).unwrap() )],
        ]
    }
}

fn draw_card(card_visual: &Vec<&'static str>) -> String {
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

fn load_cards_str_front(deck: & mut VisualDeck) {

    deck.add(CardType::Joker, 0, make_str_card!(
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
    deck.add(CardType::Joker, 0, make_str_card!(
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
    deck.add(CardType::Sword, 12, make_str_card!(
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
    deck.add(CardType::Sword, 11, make_str_card!(
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
    deck.add(CardType::Sword, 10, make_str_card!(
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
    deck.add(CardType::Sword, 9, make_str_card!(
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
    deck.add(CardType::Sword, 8, make_str_card!(
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
    deck.add(CardType::Sword, 7, make_str_card!(
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
    deck.add(CardType::Sword, 6, make_str_card!(
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
    deck.add(CardType::Sword, 5, make_str_card!(
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
    deck.add(CardType::Sword, 4, make_str_card!(
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
    deck.add(CardType::Sword, 3, make_str_card!(
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
    deck.add(CardType::Sword, 2, make_str_card!(
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
    deck.add(CardType::Sword, 1, make_str_card!(
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
    deck.add(CardType::Club, 12, make_str_card!(
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
    deck.add(CardType::Club, 11, make_str_card!(
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
    deck.add(CardType::Club, 10, make_str_card!(
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
    deck.add(CardType::Club, 9, make_str_card!(
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
    deck.add(CardType::Club, 8, make_str_card!(
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
    deck.add(CardType::Club, 7, make_str_card!(
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
    deck.add(CardType::Club, 6, make_str_card!(
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
    deck.add(CardType::Club, 5, make_str_card!(
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
    deck.add(CardType::Club, 4, make_str_card!(
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
    deck.add(CardType::Club, 3, make_str_card!(
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
    deck.add(CardType::Club, 2, make_str_card!(
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
    deck.add(CardType::Club, 1, make_str_card!(
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

    deck.add(CardType::Gold, 12, make_str_card!(
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
    deck.add(CardType::Gold, 11, make_str_card!(
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
    deck.add(CardType::Gold, 10, make_str_card!(
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
    deck.add(CardType::Gold, 9, make_str_card!(
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
    deck.add(CardType::Gold, 8, make_str_card!(
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
    deck.add(CardType::Gold, 7, make_str_card!(
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
    deck.add(CardType::Gold, 6, make_str_card!(
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
    deck.add(CardType::Gold, 5, make_str_card!(
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
    deck.add(CardType::Gold, 4, make_str_card!(
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
    deck.add(CardType::Gold, 3, make_str_card!(
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
    deck.add(CardType::Gold, 2, make_str_card!(
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
    deck.add(CardType::Gold, 1, make_str_card!(
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
    deck.add(CardType::Cup, 12, make_str_card!(
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
    deck.add(CardType::Cup, 11, make_str_card!(
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
    deck.add(CardType::Cup, 10, make_str_card!(
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
    deck.add(CardType::Cup, 9, make_str_card!(
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
    deck.add(CardType::Cup, 8, make_str_card!(
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
    deck.add(CardType::Cup, 7, make_str_card!(
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
    deck.add(CardType::Cup, 6, make_str_card!(
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
    deck.add(CardType::Cup, 5, make_str_card!(
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
    deck.add(CardType::Cup, 4, make_str_card!(
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
    deck.add(CardType::Cup, 3, make_str_card!(
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
    deck.add(CardType::Cup, 2, make_str_card!(
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
    deck.add(CardType::Cup, 1, make_str_card!(
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
}
