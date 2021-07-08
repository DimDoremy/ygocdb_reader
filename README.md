# Ygopro cdb reader

ygopro游戏数据库搜索的Rust命令行工具，为方便检查srvpro服务器数据库的卡片更新设计。

```
Ygopro-cdb Command Reader 0.1.0    
Dim Doremy <DimDoremy@zoho.com.cn> 
Rust编写的命令行ygopro的cdb读取工具

USAGE:
    ygocdb_reader.exe [FLAGS] [OPTIONS] --file <FILE>

FLAGS:
    -H, --hyper      开启高级查询，支持多参数同时查询
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --atk <ATK>          通过攻击力搜索相等的卡
        --af <ATKDOWN>       通过攻击力搜索小于的卡
        --au <ATKUP>         通过攻击力搜索大于的卡
    -c, --code <CODE>        通过卡密搜索
    -d, --def <DEF>          通过守备力搜索相等的卡
        --dd <DEFDOWN>       通过守备力搜索小于的卡
        --du <DEFUP>         通过守备力搜索大于的卡
    -e, --effect <EFFECT>    通过效果描述模糊搜索
    -f, --file <FILE>        打开ygopro的cdb数据库文件
    -n, --name <NAME>        通过名称模糊搜索
```