use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<Error>>;

// Stolen from burntsushi's AOC day 3 solution
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

struct Node {
    metadata: Vec<u32>,
    children: Vec<Node>
}

impl Node {
    fn from_input(input: &mut VecDeque<u32>) -> Node {
        let num_children = input.pop_front().unwrap();
        let num_meta = input.pop_front().unwrap();
        let children: Vec<Node> = (0..num_children)
            .map(|_| Node::from_input(input))
            .collect();
        let metadata: Vec<u32> = (0..num_meta)
            .map(|_| input.pop_front().unwrap())
            .collect();
        Node {
            metadata: metadata,
            children: children,
        }
    }

    fn sum_metadata(&self) -> u32 {
        let mut sum = self.metadata.iter().sum();
        sum += self.children.iter()
            .map(|c| c.sum_metadata())
            .sum::<u32>();
        sum
    }

    fn value(&self) -> u32 {
        if self.children.len() == 0 {
            self.metadata.iter().sum()
        } else {
            self.metadata.iter()
                .map(|idx| {
                    match self.children.get(*idx as usize - 1) {
                        Some(c) => c.value(),
                        None    => 0
                    }
                }).sum()
        }
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let line = match line_iter.next() {
        Some(l) => Ok(l?),
        None    => err!("No tree specification passed.")
    }?;
    let mut tree_spec: VecDeque<u32> = line.split_whitespace()
        .map(|c| Ok(c.parse::<u32>()?))
        .collect::<Result<_>>()?;
    let root_node = Node::from_input(&mut tree_spec);
    println!("Sum of metadata: {}", root_node.sum_metadata());
    println!("Value of root node: {}", root_node.value());
    Ok(())
}
