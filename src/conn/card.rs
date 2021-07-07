#[derive(Clone)]
pub struct Card {
    pub id: u64,
    pub name: String,
    pub atk: i32,
    pub def: i32,
    pub desc: String,
}

impl Card {
    pub fn new() -> Self {
        Card {
            id: 0,
            name: "".to_string(),
            atk: -3,
            def: -3,
            desc: "".to_string(),
        }
    }

    pub fn from_data(id: u64, name: String, atk: i32, def: i32, desc: String) -> Self {
        Card {
            id,
            name,
            atk,
            def,
            desc,
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{},{}/{},{})",
            self.id, self.name, self.atk, self.def, self.desc
        )
    }
}
