use rusqlite::{Connection, Error, OpenFlags};

pub struct Conn {
    pub(crate) conn: rusqlite::Connection,
}

impl Conn {
    // 新建内存数据库
    pub fn new() -> Self {
        Conn {
            conn: Connection::open_in_memory().unwrap(),
        }
    }

    // 连接数据库
    pub fn from(path: String) -> Self {
        Conn {
            conn: match Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE) {
                Ok(x) => x,
                Err(e) => panic!("{:?}", e),
            },
        }
    }

    // 检查是否是ygopro的数据库
    pub fn check_db(&self) -> Result<bool, Error> {
        let ans = vec!["datas", "texts"];
        let check_sql = "SELECT name from sqlite_master WHERE type='table' order by name";
        let mut stmt = self.conn.prepare(check_sql)?;
        let results = stmt.query_map([], |row| -> Result<String, Error> { Ok(row.get(0)?) })?;

        for (i, rs) in results.enumerate() {
            if let Ok(x) = rs {
                if ans[i] != x {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}