
#[derive(Clone)]
#[derive(Debug)]
pub struct Selector {
    pub selector_type: SelectorType,
    pub body: String
}

#[derive(Clone)]
#[derive(Debug)]
pub enum SelectorType {
    PlayerAll,
    EntitiesAll,
    PlayerClosest,
    PlayerRandom,
    Caller
}

impl Selector {
    pub fn new(type_char: char, body: String) -> Result<Selector, String> {
        let selector_type = match type_char {
            'a' => SelectorType::PlayerAll,
            'e' => SelectorType::EntitiesAll,
            'p' => SelectorType::PlayerClosest,
            'r' => SelectorType::PlayerRandom,
            's' => SelectorType::Caller,
            _ => return Err(String::from("Unable to parse"))
        };

        Ok(Selector {
            selector_type: selector_type,
            body: body
        })
    }

    pub fn selects_multiple(&self) -> bool {
        
        let has_single_modifier = self.body
        .trim_matches(|a|matches!(a,'[')||matches!(a,']'))
        .split(',')
        .map(
            |item| {
                let mut split = item.split('=');
                matches!(split.next(),Some("limit")) &&
                matches!(split.next_back().map(|b|b.parse::<i32>()),Some(Ok(1)))
            }
        ).fold(false,|a,e|a||e);

        match self.selector_type {
            SelectorType::PlayerAll => has_single_modifier,
            SelectorType::EntitiesAll => has_single_modifier,
            SelectorType::PlayerClosest => false,
            SelectorType::PlayerRandom => false,
            SelectorType::Caller => false,
        }
    }
}