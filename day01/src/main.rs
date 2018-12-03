use std::collections::HashSet;
use std::error;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn main() -> Result<()> {
    let mut freq: i64 = 0;
    let stdin = io::stdin();
    let freq_changes: Vec<i64> = stdin.lock().lines()
        .map(|l| Ok(l?.parse()?))
        .collect::<std::result::Result<_, Box<error::Error>>>()?;

    for change in &freq_changes {
        freq += change;
    }
    println!("Frequency after first round of changes: {}", freq);

    freq = 0;
    let mut seen_freqs = HashSet::new();
    for change in freq_changes.iter().cycle() {
        freq += change;
        if !seen_freqs.insert(freq) {
            println!("Frequency encountered twice: {}", freq);
            break;
        }
    }
    Ok(())
}
