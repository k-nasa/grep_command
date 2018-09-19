use std::env::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate regex;
use regex::Regex;

/// 引数パラメーターがおかしいときに呼び出す
fn usage() {
    println!("usage: grep PATTERN FILEPATH");
}

fn main() {
    let pattern = match args().nth(1) {
        Some(pattern) => pattern,
        None => {
            usage();
            return;
        }
    };

    let reg = match Regex::new(&pattern) {
        Ok(reg) => reg,
        Err(e) => {
            println!("Invalid regex {}: {}", pattern, e);
            return;
        }
    };

    let filename = match args().nth(2) {
        Some(name) => name,
        None => {
            usage();
            return;
        }
    };

    let file = match File::open(&filename) {
        // NOTE ?を使ってスッキリ書きたいけどエラー時の出力がほしい
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}:{}", filename, e);
            return;
        }
    };

    // ファイルIOが遅いのでBufReaderを使う
    let input = BufReader::new(file);

    for line in input.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("An error read a line {}", e);
                return;
            }
        };

        if reg.is_match(&line) {
            println!("{}", line);
        }
    }
}
