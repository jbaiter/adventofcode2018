use std::collections::HashMap;
use std::error;
use std::io;
use std::io::BufRead;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn checksum_boxes(box_ids: &Vec<String>) -> u64 {
    let mut two_ids = 0;
    let mut three_ids = 0;
    for box_id in box_ids {
        let mut char_counts = HashMap::new();
        for chr in box_id.chars() {
            *char_counts.entry(chr).or_insert(0) += 1;
        }
        let twos = char_counts.values().filter(|cnt| **cnt == 2).count();
        let threes = char_counts.values().filter(|cnt| **cnt == 3).count();
        if twos > 0 {
            two_ids += 1;
        }
        if threes > 0 {
            three_ids += 1;
        }
    }
    return two_ids * three_ids;
}

fn string_dist<'a>(a: &'a str, b: &'a str) -> u64 {
    let mut dist = a.chars().zip(b.chars()).map(|(x, y)| (x != y) as u64).sum();
    dist += (a.len() as i64 - b.len() as i64).abs() as u64;
    return dist;
}

fn find_right_boxes(box_ids: &Vec<String>) -> Option<(String, String)> {
    for a in box_ids {
        for b in box_ids {
            if string_dist(a, b) == 1 {
                return Some((a.to_string(), b.to_string()))
            }
        }
    }
    None
}

fn get_common_chars<'a>(a: &'a str, b: &'a str) -> String {
    a.chars().zip(b.chars()).filter(|(x, y)| x == y).map(|(x, _)| x).collect()
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let box_ids: Vec<String> = stdin.lock().lines()
        .map(|l| Ok(l?.to_string()))
        .collect::<Result<_>>()?;
    let checksum = checksum_boxes(&box_ids);
    println!("List checksum: {}", checksum);
    match find_right_boxes(&box_ids) {
        Some((a, b)) => println!("Common characters: {}",
                                 get_common_chars(&a, &b)),
        None => println!("No boxes found!")
    };
    Ok(())
}
