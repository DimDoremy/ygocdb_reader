use rusqlite::Error;

use crate::conn::card::Card;

use super::conn::Conn;

pub enum Operation {
    EQ, // 相等
    GT, // 大于
    LT, // 小于
}

pub trait Select {
    // 查询所有有价值的信息,通过攻击力
    fn select_atk(&self, atk: u32, opt: Operation) -> Result<(), Error>;
    // 查询所有有价值的信息,通过守备力
    fn select_def(&self, def: u32, opt: Operation) -> Result<(), Error>;
    // 查询所有有价值的信息,通过卡密
    fn select_code(&self, code: u64) -> Result<(), Error>;
    // 查询所有有价值的信息,通过效果
    fn select_effect(&self, effect: String) -> Result<(), Error>;
    // 查询所有有价值的信息,通过名称
    fn select_name(&self, name: String) -> Result<(), Error>;
    // 高级查询
    fn select_hyper(&self, card: Card, atk_opt: Operation, def_opt: Operation)
        -> Result<(), Error>;
}

impl Select for Conn {
    fn select_atk(&self, atk: u32, opt: Operation) -> Result<(), Error> {
        let sql = format!("SELECT d.id,name,atk,def,desc FROM datas AS d INNER JOIN texts AS t ON d.id=t.id WHERE {};",  match opt {
            Operation::EQ => "atk=?",
            Operation::GT => "atk>?",
            Operation::LT => "atk<?", 
        });
        let mut stmt = self.conn.prepare(&sql)?;
        let results = stmt.query_map([atk], |row| {
            Ok(Card::from_data(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        for rs in results {
            println!("{:?}", rs?);
        }

        Ok(())
    }

    fn select_def(&self, def: u32, opt: Operation) -> Result<(), Error> {
        let sql = format!("SELECT d.id,name,atk,def,desc FROM datas AS d INNER JOIN texts AS t ON d.id=t.id WHERE {};",  match opt {
            Operation::EQ => "def=?",
            Operation::GT => "def>?",
            Operation::LT => "def<?", 
        });
        let mut stmt = self.conn.prepare(&sql)?;
        let results = stmt.query_map([def], |row| {
            Ok(Card::from_data(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        for rs in results {
            println!("{:?}", rs?);
        }

        Ok(())
    }

    fn select_code(&self, code: u64) -> Result<(), Error> {
        let sql = "SELECT d.id,name,atk,def,desc FROM datas AS d INNER JOIN texts AS t ON d.id=t.id WHERE d.id=?;";
        let mut stmt = self.conn.prepare(&sql)?;
        let results = stmt.query_map([code], |row| {
            Ok(Card::from_data(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        for rs in results {
            println!("{:?}", rs?);
        }

        Ok(())
    }

    fn select_effect(&self, effect: String) -> Result<(), Error> {
        let effect = format!("%{}%", effect);
        let sql = "SELECT d.id,name,atk,def,desc FROM datas AS d INNER JOIN texts AS t ON d.id=t.id WHERE desc LIKE ?;";
        let mut stmt = self.conn.prepare(&sql)?;
        let results = stmt.query_map([effect], |row| {
            Ok(Card::from_data(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        for rs in results {
            println!("{:?}", rs?);
        }

        Ok(())
    }

    fn select_name(&self, name: String) -> Result<(), Error> {
        let name = format!("%{}%", name);
        let sql = "SELECT d.id,name,atk,def,desc FROM datas AS d INNER JOIN texts AS t ON d.id=t.id WHERE name LIKE ?;";
        let mut stmt = self.conn.prepare(&sql)?;
        let results = stmt.query_map([name], |row| {
            Ok(Card::from_data(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        for rs in results {
            println!("{:?}", rs?);
        }

        Ok(())
    }

    fn select_hyper(
        &self,
        card: Card,
        atk_opt: Operation,
        def_opt: Operation,
    ) -> Result<(), Error> {
        // 定义条件判断
        let mut str = String::new();
        // 如果id存在
        if card.id > 0 {
            str = format!("d.id={}", card.id);
        }
        // 如果name存在
        if card.name.len() > 0 {
            // 判断当前条件长度是否为0
            if str.len() == 0 {
                str = format!("name LIKE '%{}%'", card.name);
            } else {
                str = format!("{} AND name LIKE '%{}%'", str, card.name);
            }
        }
        // 如果atk存在
        if card.atk >= -2 {
            if str.len() == 0 {
                match atk_opt {
                    Operation::EQ => str = format!("atk={}", card.atk),
                    Operation::GT => str = format!("atk>{}", card.atk),
                    Operation::LT => str = format!("atk<{}", card.atk),
                }
            } else {
                match atk_opt {
                    Operation::EQ => str = format!("{} AND atk={}", str, card.atk),
                    Operation::GT => str = format!("{} AND atk>{}", str, card.atk),
                    Operation::LT => str = format!("{} AND atk<{}", str, card.atk),
                }
            }
        }
        // 如果def存在
        if card.def >= -2 {
            if str.len() == 0 {
                match def_opt {
                    Operation::EQ => str = format!("def={}", card.def),
                    Operation::GT => str = format!("def>{}", card.def),
                    Operation::LT => str = format!("def<{}", card.def),
                }
            } else {
                match def_opt {
                    Operation::EQ => str = format!("{} AND def={}", str, card.def),
                    Operation::GT => str = format!("{} AND def>{}", str, card.def),
                    Operation::LT => str = format!("{} AND def<{}", str, card.def),
                }
            }
        }
        // 如果desc存在
        if card.desc.len() > 0 {
            if str.len() == 0 {
                str = format!("desc LIKE '%{}%'", card.desc);
            } else {
                str = format!("{} AND desc LIKE '%{}%'", str, card.desc);
            }
        }
        if str.len() > 0 {
            str = format!("WHERE {}", str);
        }

        // 查询处理
        let sql = format!(
            "SELECT d.id,name,atk,def,desc FROM datas d INNER JOIN texts t ON d.id=t.id {};",
            str
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let results = stmt.query_map([], |row| {
            Ok(Card::from_data(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        for rs in results {
            println!("{:?}", rs?);
        }
        Ok(())
    }
}
