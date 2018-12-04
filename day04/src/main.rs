#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::error::Error;
use std::str::FromStr;

use regex::Regex;

type Result<T> = std::result::Result<T, Box<Error>>;

// Stolen from burntsushi's AOC day 3 solution
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Timestamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32
}

#[derive(Debug)]
enum GuardEvent {
    BeginShift(u32),
    Wake,
    Sleep
}

impl FromStr for GuardEvent {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<GuardEvent> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "Guard" => Ok(GuardEvent::BeginShift(parts[1][1..].parse()?)),
            "wakes" => Ok(GuardEvent::Wake),
            "falls" => Ok(GuardEvent::Sleep),
            _       => err!("Unknown event {}", s)
        }
    }
}

#[derive(Debug)]
struct GuardLogEntry {
    event: GuardEvent,
    time: Timestamp,
}

impl FromStr for GuardLogEntry {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<GuardLogEntry> {
        lazy_static! {
            static ref LOG_RE: Regex = Regex::new(
                r"^\[(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2}) (?P<h>\d{2}):(?P<M>\d{2})\] (?P<evt>.+)$").unwrap();
        }
        let caps = match LOG_RE.captures(s) {
            Some(caps) => Ok(caps),
            None       => err!("Bad log entry, could not parse: {}", s)
        }?;
        Ok(GuardLogEntry {
            event: caps["evt"].parse()?,
            time: Timestamp {
                year: caps["y"].parse()?, month: caps["m"].parse()?,
                day: caps["d"].parse()?, hour: caps["h"].parse()?,
                minute: caps["M"].parse()? } })
    }
}

#[derive(Debug)]
struct GuardLog {
    entries: Vec<GuardLogEntry>,
}

impl GuardLog {
    fn new(mut entries: Vec<GuardLogEntry>) -> Result<GuardLog> {
        entries.sort_unstable_by(|a, b| a.time.cmp(&b.time));
        if entries.len() == 0 {
            return err!("There must be at least one log entry!");
        }
        match entries[0].event {
            GuardEvent::BeginShift(_) => Ok(GuardLog { entries: entries }),
            _ => err!("First log entry must be begin of a shift!")
        }
    }

    fn get_sleep_times(&self) -> HashMap<u32, [u32; 60]> {
        let mut guard_times: HashMap<u32, [u32; 60]> = HashMap::new();
        let mut sleep_start: u32 = 0;
        let mut current_times: Option<&mut [u32; 60]> = None;
        for entry in &self.entries {
            match entry.event {
                GuardEvent::BeginShift(guard_id) => {
                    if !guard_times.contains_key(&guard_id) {
                        let times = [0; 60];
                        guard_times.insert(guard_id, times);
                    }
                    current_times = guard_times.get_mut(&guard_id);
                },
                GuardEvent::Sleep => sleep_start = entry.time.minute,
                GuardEvent::Wake  => {
                    for minute in sleep_start..entry.time.minute {
                        current_times.as_mut()
                            .map_or((), |ts| ts[minute as usize] += 1);
                    }
                }
            }
        }
        guard_times
    }

    fn strategy_one(&self) -> (u32, u32) {
        let sleep_times = self.get_sleep_times();
        let (guard_id, ts) = sleep_times.iter()
            .max_by(|(_, a), (_, b)| {
                a.iter().sum::<u32>().cmp(&b.iter().sum::<u32>())
            }).unwrap();
        let sleepy_minute = ts.iter().enumerate()
            .max_by_key(|&(_, v)| v).unwrap().0 as u32;
        (*guard_id, sleepy_minute)
    }

    fn strategy_two(&self) -> (u32, u32) {
        let mut sleepiest_minutes: HashMap<u32, (u32, u32)> = HashMap::new();
        for (guard, times) in self.get_sleep_times() {
            let (min, cnt) = times.iter().enumerate()
                .max_by_key(|&(_, v)| v).unwrap();
            sleepiest_minutes.insert(guard, (min as u32, *cnt));
        }
        let (&long_guard, &(min, _)) = sleepiest_minutes.iter()
            .max_by_key(|(_, (_, cnt))| cnt).unwrap();
        (long_guard, min)
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let entries: Vec<GuardLogEntry> = stdin.lock().lines()
        .map(|l| l?.parse())
        .collect::<Result<_>>()?;
    let log = GuardLog::new(entries)?;
    let (gid_one, min_one) = log.strategy_one();
    println!("Strategy I checksum: {} * {} = {}",
             gid_one, min_one, gid_one * min_one);
    let (gid_two, min_two) = log.strategy_two();
    println!("Strategy II checksum: {} * {} = {}",
             gid_two, min_two, gid_two * min_two);
    Ok(())
}
