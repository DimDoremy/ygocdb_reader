#[derive(Clone)]
pub struct Card {
    pub id: u64,
    pub name: String,
    pub types: u64,
    pub atk: i32,
    pub def: i32,
    pub desc: String,
}

impl Card {
    pub fn new() -> Self {
        Card {
            id: 0,
            name: "".to_string(),
            types: 0,
            atk: -3,
            def: -3,
            desc: "".to_string(),
        }
    }

    pub fn from_data(id: u64, name: String, types: u64, atk: i32, def: i32, desc: String) -> Self {
        Card {
            id,
            name,
            types,
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
            "({},{},{},{}/{},{})",
            self.id, self.name, self.types, self.atk, self.def, self.desc
        )
    }
}