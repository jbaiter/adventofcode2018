#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::{self, BufRead};
use std::collections::HashMap;
use std::error;
use std::fmt;

use regex::Regex;


type Result<T> = ::std::result::Result<T, Box<error::Error>>;

#[derive(Debug, Clone)]
struct ParseError {
    input: String
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse: {}", self.input)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "failed to parse"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

struct PositionIter<'a> {
    claim: &'a Claim,
    cur_pos: (u32, u32)
}

impl<'a> Iterator for PositionIter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        let (x, y) = self.cur_pos;
        if x == self.claim.lrx && y < self.claim.lry {
            // Increment row
            self.cur_pos = (self.claim.ulx, y + 1);
        } else if (x + 1) <= self.claim.lrx {
            // Increment column
            self.cur_pos = (x + 1, y);
        } else {
            return None;
        }
        return Some(self.cur_pos);
    }
}

#[derive(Debug, Hash, PartialEq)]
struct Claim {
    id: u32,
    ulx: u32,
    uly: u32,
    lrx: u32,
    lry: u32
}

impl Claim {
    pub fn parse<'a>(spec: &'a str) -> Result<Claim> {
        lazy_static! {
            // #<id> @ <x>,<y>: <w>,<h>
            static ref CLAIM_RE: Regex = Regex::new(
                r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)$").unwrap();
        }
        let make_err = || ParseError { input: spec.to_string() };
        let caps = CLAIM_RE.captures(spec).ok_or(make_err())?;
        let x: u32 = caps.name("x").ok_or(make_err())?.as_str().parse()?;
        let y: u32 = caps.name("y").ok_or(make_err())?.as_str().parse()?;
        let w: u32 = caps.name("w").ok_or(make_err())?.as_str().parse()?;
        let h: u32 = caps.name("h").ok_or(make_err())?.as_str().parse()?;

        Ok(Claim {
            id: caps.name("id").ok_or(make_err())?.as_str().parse()?,
            ulx: x,
            uly: y,
            lrx: x + w - 1,
            lry: y + h - 1,
        })
    }

    pub fn positions(&self) -> PositionIter {
        PositionIter {
            claim: self,
            cur_pos: (self.ulx - 1, self.uly)
        }
    }
}


fn main() -> Result<()> {
    let mut positions: HashMap<(u32, u32), u32> = HashMap::new();
    let stdin = io::stdin();
    let claims: Vec<Claim> = stdin.lock().lines()
        .map(|l| Claim::parse(&l?))
        .collect::<Result<_>>()?;
    for claim in &claims {
        for pos in claim.positions() {
            *positions.entry(pos).or_insert(0) += 1;
        }
    }
    println!("Positions with overlap: {}",
             positions.values().filter(|&&v| v >= 2).count());
    let non_overlap = claims.iter()
        .filter(|&c| c.positions().filter(|pos| positions[pos] > 1).next().is_none())
        .next();
    match non_overlap {
        Some(claim) => println!("Claim with no overlaps: {}", claim.id),
        None => println!("No claims without overlaps found!")
    }
    Ok(())
}
