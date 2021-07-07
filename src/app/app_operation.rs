use clap::ArgMatches;
use rusqlite::Error;

use crate::conn::{card::Card, conn::Conn, select::{Operation, Select}};

pub fn search(matches: ArgMatches, _conns: &mut Conn) -> Result<(), Error> {
    // 如果没开启高级查询
    // -a
    match matches.value_of("ATK") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                _conns.select_atk(x, Operation::EQ)?;
                return Ok(());
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // --au
    match matches.value_of("ATKUP") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                _conns.select_atk(x, Operation::GT)?;
                return Ok(());
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // --ad
    match matches.value_of("ATKDOWN") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                _conns.select_atk(x, Operation::LT)?;
                return Ok(());
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // -d
    match matches.value_of("DEF") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                _conns.select_def(x, Operation::EQ)?;
                return Ok(());
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // --du
    match matches.value_of("DEFUP") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                _conns.select_def(x, Operation::GT)?;
                return Ok(());
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // --dd
    match matches.value_of("DEFDOWN") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                _conns.select_def(x, Operation::LT)?;
                return Ok(());
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // -c
    match matches.value_of("CODE") {
        Some(x) => match x.parse::<u64>() {
            Ok(x) => {
                _conns.select_code(x)?;
                return Ok(());
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // -e
    match matches.value_of("EFFECT") {
        Some(x) => {
            _conns.select_effect(x.to_string())?;
            return Ok(());
        }
        None => (),
    };
    // -n
    match matches.value_of("NAME") {
        Some(x) => {
            _conns.select_name(x.to_string())?;
            return Ok(());
        }
        None => (),
    };
    Ok(())
}

pub fn search_hyper(matches: ArgMatches, _conns: &mut Conn) -> Result<(), Error> {
    let mut atk_opt: Operation = Operation::EQ;
    let mut def_opt: Operation = Operation::EQ;
    
    let mut param = Vec::new();
    // -a
    match matches.value_of("ATK") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                param.push(("atk", x.to_string()));
                atk_opt = Operation::EQ;
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // --au
    match matches.value_of("ATKUP") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                param.push(("atk", x.to_string()));
                atk_opt = Operation::GT;
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // --ad
    match matches.value_of("ATKDOWN") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                param.push(("atk", x.to_string()));
                atk_opt = Operation::LT;
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // -d
    match matches.value_of("DEF") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                param.push(("def", x.to_string()));
                def_opt = Operation::EQ;
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // --du
    match matches.value_of("DEFUP") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                param.push(("def", x.to_string()));
                def_opt = Operation::GT;
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // --dd
    match matches.value_of("DEFDOWN") {
        // 判断是否是数字
        Some(x) => match x.parse::<u32>() {
            Ok(x) => {
                param.push(("def", x.to_string()));
                def_opt = Operation::LT;
            }
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // -c
    match matches.value_of("CODE") {
        Some(x) => match x.parse::<u64>() {
            Ok(x) => param.push(("code", x.to_string())),
            Err(e) => panic!("Err: {:?}", e),
        },
        None => (),
    };
    // -e
    match matches.value_of("EFFECT") {
        Some(x) => param.push(("effect", x.to_string())),
        None => (),
    };
    // -n
    match matches.value_of("NAME") {
        Some(x) => param.push(("name", x.to_string())),
        None => (),
    };

    let mut card_search = Card::new();
    for (k, v) in param {
        match k {
            "atk" => card_search.atk = v.parse::<i32>().unwrap(),
            "def" => card_search.def = v.parse::<i32>().unwrap(),
            "code" => card_search.id = v.parse::<u64>().unwrap(),
            "effect" => card_search.desc = v,
            "name" => card_search.name = v,
            _ => (),
        }
    }
    _conns.select_hyper(card_search, atk_opt, def_opt)
}