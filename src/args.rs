/*-------------------------------
            args.rs

  起動引数を取得して、
  場合に応じたオプションをつける
  
  * print_usage()
    * ヘルプ表示用の関数
  * print_version()
    * version表示用の関数
  * new()
    * 現状ではデバッグモード用の関数
-------------------------------*/
use std;

const USAGE: &'static str = "USAGE
dodge rock game

dodge_rock (-h | --help)
dodge_rock (-v | --version)
dodge_rock (-d | --debug)

Options:
  -h --help     Show this screen.
  -v --version  Show version.
  -d --debug    Run game with debug mode.
";

// build時にCargo.tomlから名前とバージョンを組み込ませる
const OWN_NAME: &'static str = env!("CARGO_PKG_NAME");
const OWN_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// helpを表示
fn print_usage() {
    println!("{}", USAGE);
}

/// versionを表示
fn print_version() {
    println!("{} v{}", OWN_NAME, OWN_VERSION);
}

/// 起動引数の読み込みと分析
pub fn new() -> bool {
    // 引数なしの場合は早期終了、エラー回避。
    if std::env::args().len() == 1 {
        println!("引数なしの通常モード");
        return false;
    }
    
    let args: Vec<String> = std::env::args().skip(1).collect();
    let first_arg = args[0].as_str();
    
    let is_debug = match first_arg {
        "-h" | "--help" => {
            print_usage();
            std::process::exit(0);
        }
        
        "-v" | "--version" => {
            print_version();
            std::process::exit(0);
        }
        
        "-d" | "--debug" => {
            println!("でばっぐもーど");
            true
        }
        
        _ => {
            println!("通常起動");
            false
        }
    };
    
    is_debug
}
