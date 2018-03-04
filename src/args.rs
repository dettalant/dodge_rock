/*-------------------------------
            args.rs

  起動引数を取得して、
  場合に応じたオプションをつける
  
  * print_usage()  : ヘルプ表示用の関数
  * print_version(): version表示用の関数
  * impl Args
    * new()        : 外部から呼び出す関数。内部でargs_check()を呼ぶ。
    * default()    : struct Argsを初期化するやつ
    * args_check() : env::args()の値を見て、適切なモードを指定する
  
  * struct Args
    * flag_debug   : debug modeかどうかを判定する変数

-------------------------------*/
use std::{self, env};

const USAGE: &'static str = "  \
  Description:
    dodge rock game
  
  USAGE:
    dodge_rock (-h | --help)
    dodge_rock (-v | --version)
    dodge_rock (-d | --debug)

  Options:
    -h --help     Show this screen.
    -v --version  Show version.
    -d --debug    Run game with debug mode.";

// build時にCargo.tomlから名前とバージョンを組み込ませる
const OWN_NAME: &'static str = env!("CARGO_PKG_NAME");
const OWN_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// USAGEメッセージを表示。内部用。
fn print_usage() {
    println!("{}", USAGE);
}

/// versionを表示。内部用。
fn print_version() {
    println!("{} v{}", OWN_NAME, OWN_VERSION);
}

#[derive(Debug, Default)]
pub struct Args {
    pub flag_debug: bool,
}

impl Args {
    /// 起動引数の読み込みと分析
    pub fn new() {
        // Args struct用の各種変数初期化
        let mut args:Args = Default::default();

        // 引数なしの場合は早期終了、エラー回避。
        if env::args().len() == 1 {
            // Do nothing
        } else {
            args.args_check();
        }
        
        args.set_to_env_var();
    }
    
    /// 内部用。env::args()を見て、適切な引数が使われていたら作動する。
    fn args_check(&mut self) {
        // 起動引数を取得
        let env_args: Vec<String> = env::args().skip(1).collect();
        let first_arg = env_args[0].as_str();
        
        match first_arg {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            
            "-v" | "--version" => {
                print_version();
                std::process::exit(0);
            }
            
            "-d" | "--debug" => {
                self.flag_debug = true;
            }
            
            _ => (),
        } // match end
    }
    
    /// ゲームの起動引数に応じて、起動モードを環境変数に指定
    fn set_to_env_var(&self) {
        let game_activate_mode = "GAME_ACTIVATE_MODE";
        
        if self.flag_debug {
            let debug = "DEBUG_MODE";
            env::set_var(game_activate_mode, debug);
        } else {
            let normal = "NORMAL_MODE";
            env::set_var(game_activate_mode, normal);
        }
    }
}
