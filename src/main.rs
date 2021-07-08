use crate::{app::{app_builder::app_builder, app_operation::{search, search_hyper}}, conn::{
    conn::Conn,
}};

pub mod conn;
pub mod app;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), rusqlite::Error> {
    let matches = app_builder(VERSION);
    let mut _conns: Conn = Conn::new();

    // -f
    match matches.value_of("FILE") {
        Some(x) => {
            _conns = Conn::from(x.to_string());
            if let Ok(check) = _conns.check_db() {
                if !check {
                    panic!("Error: 打开数据库不是ygopro的数据库");
                }
            }
        }
        None => panic!("Error: 未找到数据库"),
    };

    // 如果开启高级查询
    if matches.is_present("HYPER") {
        search_hyper(matches, &mut _conns)
    } else {
        search(matches, &mut _conns)
    }
}

