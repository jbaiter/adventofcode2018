use std::io::{self, Read};

fn toggle_case(c: char) -> char {
    if c.is_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}

fn reduce(chain: &String) -> String {
    chain.chars().fold(String::new(), |mut r, c| {
        if r.ends_with(toggle_case(c)) {
            r.pop();
        } else {
            r.push(c);
        }
        r
    })

}

fn find_maximum_reduction(chain: &String) -> String {
    "abcdefghijklmnopqrstuvwxyz".chars()
        .map(|c| -> String {
            chain.chars()
                .filter(|&x| x != c && x != toggle_case(c))
                .collect() })
        .map(|c| reduce(&c))
        .min_by_key(|c| c.len())
        .unwrap()
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
