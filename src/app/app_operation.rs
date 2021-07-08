use clap::ArgMatches;
use rusqlite::Error;

use crate::conn::{
    card::Card,
    conn::Conn,
    select::{Operation, Select},
};

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
    // -k
    match matches.value_of("KIND") {
        Some(kind) => {
            let kind_vec = kind.split("-");
            let mut kind_code: u64 = 0;
            for k in kind_vec {
                match k {
                    "怪兽" => kind_code += 0x1,
                    "魔法" => kind_code += 0x2,
                    "陷阱" => kind_code += 0x4,
                    "通常" => kind_code += 0x10,
                    "效果" => kind_code += 0x20,
                    "融合" => kind_code += 0x30,
                    "灵魂" => kind_code += 0x200,
                    "同盟" => kind_code += 0x400,
                    "二重" => kind_code += 0x800,
                    "调整" => kind_code += 0x1_000,
                    "同调" => kind_code += 0x2_000,
                    "反转" => kind_code += 0x200_000,
                    "卡通" => kind_code += 0x400_000,
                    "超量" => kind_code += 0x800_000,
                    "灵摆" => kind_code += 0x1_000_000,
                    "特殊召唤" => kind_code += 0x2_000_000,
                    "链接" => kind_code += 0x4_000_000,
                    "仪式" => kind_code += 0x80,
                    "速攻" => kind_code += 0x10_000,
                    "永续" => kind_code += 0x20_000,
                    "装备" => kind_code += 0x40_000,
                    "场地" => kind_code += 0x80_000,
                    "反击" => kind_code += 0x100_000,
                    _ => panic!("输入的卡片种类不正确"),
                }
            }
            _conns.select_kind(kind_code)?;
            return Ok(());
        }
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
    // -k
    match matches.value_of("KIND") {
        Some(kind) => {
            let kind_vec = kind.split("-");
            let mut kind_code: u64 = 0;
            for k in kind_vec {
                match k {
                    "怪兽" => kind_code += 0x1,
                    "魔法" => kind_code += 0x2,
                    "陷阱" => kind_code += 0x4,
                    "通常" => kind_code += 0x10,
                    "效果" => kind_code += 0x20,
                    "融合" => kind_code += 0x30,
                    "灵魂" => kind_code += 0x200,
                    "同盟" => kind_code += 0x400,
                    "二重" => kind_code += 0x800,
                    "调整" => kind_code += 0x1_000,
                    "同调" => kind_code += 0x2_000,
                    "反转" => kind_code += 0x200_000,
                    "卡通" => kind_code += 0x400_000,
                    "超量" => kind_code += 0x800_000,
                    "灵摆" => kind_code += 0x1_000_000,
                    "特殊召唤" => kind_code += 0x2_000_000,
                    "链接" => kind_code += 0x4_000_000,
                    "仪式" => kind_code += 0x80,
                    "速攻" => kind_code += 0x10_000,
                    "永续" => kind_code += 0x20_000,
                    "装备" => kind_code += 0x40_000,
                    "场地" => kind_code += 0x80_000,
                    "反击" => kind_code += 0x100_000,
                    _ => panic!("输入的卡片种类不正确"),
                }
            }
            param.push(("type", kind_code.to_string()));
        }
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
            "type" => card_search.types = v.parse::<u64>().unwrap(),
            "code" => card_search.id = v.parse::<u64>().unwrap(),
            "effect" => card_search.desc = v,
            "name" => card_search.name = v,
            _ => (),
        }
    }
    _conns.select_hyper(card_search, atk_opt, def_opt)
}
