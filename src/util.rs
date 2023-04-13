use std::io::Write;

use color_eyre::Result;
use eyre::eyre;

pub mod color {
    #![allow(dead_code)]
    pub const NONE: &str = "\x1b[0m";
    pub const RESET: &str = "\x1b[0m";
    pub const BRIGHT: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const UNDERSCORE: &str = "\x1b[4m";
    pub const BLINK: &str = "\x1b[5m";
    pub const REVERSE: &str = "\x1b[7m";
    pub const HIDDEN: &str = "\x1b[8m";
    pub const FG_BLACK: &str = "\x1b[30m";
    pub const FG_RED: &str = "\x1b[31m";
    pub const FG_GREEN: &str = "\x1b[32m";
    pub const FG_YELLOW: &str = "\x1b[33m";
    pub const FG_BLUE: &str = "\x1b[34m";
    pub const FG_MAGENTA: &str = "\x1b[35m";
    pub const FG_CYAN: &str = "\x1b[36m";
    pub const FG_WHITE: &str = "\x1b[37m";
    pub const BG_BLACK: &str = "\x1b[40m";
    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_GREEN: &str = "\x1b[42m";
    pub const BG_YELLOW: &str = "\x1b[43m";
    pub const BG_BLUE: &str = "\x1b[44m";
    pub const BG_MAGENTA: &str = "\x1b[45m";
    pub const BG_CYAN: &str = "\x1b[46m";
    pub const BG_WHITE: &str = "\x1b[47m";
}

pub mod logger {
    #![allow(dead_code)]
    use crate::util::color;

    pub fn error(msg: &str) {
        println!("{red}error{r}: {msg}", red=color::FG_RED, r=color::RESET);
    }

    pub fn warn(msg: &str) {
        println!("{yellow}warn{r}: {msg}", yellow=color::FG_YELLOW, r=color::RESET);
    }
    
    pub fn info(msg: &str) {
        println!("{green}info{r}: {msg}", green=color::FG_GREEN, r=color::RESET);
    }

    pub fn note(msg: &str) {
        println!("{blue}note{r}: {msg}", blue=color::FG_BLUE, r=color::RESET);
    }


    pub fn help(msg: &str) {
        println!("{blue}help{r}: {msg}", blue=color::FG_CYAN, r=color::RESET);
    }
    pub fn cmd(msg: &str) {
        println!("{blue}cmd{r}: {msg}", blue=color::FG_MAGENTA, r=color::RESET);
    }

    pub fn code_block(code: &str) -> String {
        let mut ret = String::new();
        let lines = code.lines();

        for (i, line) in lines.enumerate() {
            use std::fmt::Write;
            writeln!(ret, "{}{} | {}{}",color::FG_BLUE, i + 1, line, color::RESET).unwrap();
        }
        ret
    }
    pub mod macros {
        #[macro_export] macro_rules! error { ($($arg:tt)*) => { $crate::util::logger::error(std::format_args!($($arg)*).to_string().as_str()) }; }
        #[macro_export] macro_rules! warn { ($($arg:tt)*) => {  $crate::util::logger::warn( std::format_args!($($arg)*).to_string().as_str()) }; }
        #[macro_export] macro_rules! info { ($($arg:tt)*) => {  $crate::util::logger::info( std::format_args!($($arg)*).to_string().as_str()) }; }
        #[macro_export] macro_rules! note { ($($arg:tt)*) => {  $crate::util::logger::note( std::format_args!($($arg)*).to_string().as_str()) }; }
        
        #[macro_export] macro_rules! help { ($($arg:tt)*) => {  $crate::util::logger::help( std::format_args!($($arg)*).to_string().as_str()) }; }
        #[macro_export] macro_rules! cmd { ($($arg:tt)*) => {  $crate::util::logger::cmd( std::format_args!($($arg)*).to_string().as_str()) }; }
        #[macro_export] macro_rules! code_block { ($($arg:tt)*) => {  $crate::util::logger::code_block( std::format_args!($($arg)*).to_string().as_str()) }; }
    }

}

pub struct Prompt {}

impl Prompt {
    pub fn default(msg: &str, default: &str) -> Result<String> {
        let mut s = Self::string(format!("{msg}. Leave blank for default [{default}]").as_str())?;
        if s.is_empty() { s = default.to_string(); }
        Ok(s)
    }
    
    pub fn bool(msg: &str, default: Option<bool>) -> Result<bool> {
        use crate::error;
        
        let r = if let Some(default) = default {
            if default {
                let s = Self::string(format!("{} [Y/n]", msg).as_str())?;
                match s.to_lowercase().as_str() {
                    "" | "y" | "yes" | "true" | "ye" | "t" => true,
                    "n" | "no" | "false" | "nah" | "f" => false,
                    s => {
                        error!("Unknown value {s:?}, please answer 'yes' or 'no'");
                        return Err(eyre!(""));
                    }
                }
            } else {
                let s = Self::string(format!("{} [y/N]", msg).as_str())?;
                match s.to_lowercase().as_str() {
                    "y" | "yes" | "true" | "ye" | "t" => true,
                    "" | "n" | "no" | "false" | "nah" | "f" => false,
                    s => {
                        error!("Unknown value {s:?}, please answer 'yes' or 'no'");
                        return Err(eyre!(""));
                    }
                }
            }
        } else {
            let s = Self::string(format!("{} [y/n]", msg).as_str())?;
            match s.to_lowercase().as_str() {
                "y" | "yes" | "true" | "ye" | "t" => true,
                "n" | "no" | "false" | "nah" | "f" => false,
                s => {
                    error!("Unknown value {s:?}, please answer 'yes' or 'no'");
                    return Err(eyre!(""));
                }
            }
        };
            
    
        Ok(r)
    }
    
    pub fn string(msg: &str) -> Result<String> {
        let mut line = String::new();
        print!("{}prompt{}: {msg}", color::FG_CYAN, color::RESET);
        std::io::stdout().flush()?;
        let _ = std::io::stdin().read_line(&mut line)?;
        Ok(line.replace("\n\r", "").replace('\n', ""))
    }
}