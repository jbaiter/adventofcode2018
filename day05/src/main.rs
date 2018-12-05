#[macro_use] extern crate lazy_static;
extern crate itertools;
extern crate regex;

use std::io::{self, Read};
use itertools::Itertools;
use regex::Regex;

fn reduce(chain: &String) -> String {
    lazy_static! {
        // Create pattern for pairs with opposite polarity
        static ref _PAT_STR: String = (('A' as u8)..('Z' as u8)+1)
            .flat_map(|x| vec![[x, x+32], [x+32, x]])
            .map(|cs| String::from_utf8_lossy(&cs).into_owned())
            .join("|");
        static ref REDUCTION_PAT: Regex = Regex::new(&_PAT_STR).unwrap();
    }
    let mut chain = chain.clone();
    let mut last_len = chain.len();
    loop {
        chain = REDUCTION_PAT.replace_all(&chain, "").into_owned();
        if last_len == chain.len() {
            break
        }
        last_len = chain.len()
    }
    chain
}

fn find_maximum_reduction(chain: &String) -> String {
    let mut shortest = chain.clone();
    for chr in ('A' as u8)..('Z' as u8)+1 {
        let chain_variant = chain.replace(chr as char, "")
                                 .replace((chr + 32) as char, "");
        let reduced = reduce(&chain_variant);
        if reduced.len() < shortest.len() {
            shortest = reduced;
        }
    }
    shortest
}


fn main() -> Result<(), io::Error>{
    let mut chain = String::new();
    io::stdin().read_to_string(&mut chain)?;
    chain = chain.trim().to_owned();

    let reduced = reduce(&chain);
    println!("Length of reduced chain: {}", reduced.len());
    let shortest = find_maximum_reduction(&chain);
    println!("Length of shortest reduced chain: {}", shortest.len());
    Ok(())
}
