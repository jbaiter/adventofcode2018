use std::collections::HashSet;
use std::io::{self, Read};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<Error>>;

fn next_steps(steps: &HashSet<char>,
              rules: &[(char, char)]) -> Vec<char> {
    let mut available: Vec<char> = steps.iter()
        .filter(|step| {
            rules.iter()
                .filter(|(a, b)| &b == step && steps.contains(&a))
                .count() == 0
        }).map(|c| c.clone()).collect();
    available.sort();
    available
}

fn get_steps(rules: &[(char, char)]) -> HashSet<char> {
    let mut steps: HashSet<char> = HashSet::new();
    for (a, b) in rules {
        steps.insert(*a);
        steps.insert(*b);
    }
    steps
}

fn part1(rules: &[(char, char)]) {
    let mut steps = get_steps(rules);
    let mut order: Vec<char> = Vec::new();
    while steps.len() > 0 {
        let available = next_steps(&steps, &rules);
        let next = available[0].clone();
        order.push(next);
        steps.remove(&next);
    }
    println!("{}", order.iter().collect::<String>());
}

struct Worker {
    job: char,
    busy_for: u32,
}

impl Worker {
    fn start_work(&mut self, job: char, handicap: u32) {
        self.job = job;
        self.busy_for = (job as u32) - 64 + handicap;
    }

    fn tick(&mut self) -> u32 {
        self.busy_for -= 1;
        self.busy_for
    }

    fn busy(&self) -> bool {
        self.busy_for > 0
    }
}

fn part2(rules: &[(char, char)], num_workers: u32, handicap: u32) {
    let mut workers: Vec<Worker> = (0..num_workers)
        .map(|_| Worker { job: '.', busy_for: 0 }).collect();
    let mut steps = get_steps(rules);
    let mut ticks: u32 = 0;
    let mut done: Vec<char> = Vec::new();
    while steps.len() > 0 {
        for worker in &mut workers {
            if worker.busy() {
                let ttl = worker.tick();
                if ttl == 0 {
                    steps.remove(&worker.job);
                    done.push(worker.job);
                    worker.job = '.';
                }
            }
        }
        let available_steps: Vec<char> = next_steps(&steps, rules).iter()
            .filter(|s| workers.iter().filter(|w| w.job == **s).count() == 0)
            .map(|c| c.clone())
            .collect();
        let mut available_workers: Vec<&mut Worker> = workers.iter_mut()
            .filter(|w| !w.busy())
            .collect();
        for step in &available_steps {
            match available_workers.pop() {
                Some(w) => w.start_work(*step, handicap),
                None    => break
            }
        }
        let idle_workers = workers.iter().filter(|w| !w.busy()).count();
        if idle_workers != num_workers as usize {
            ticks += 1;
        }
    }
    println!("Work took {} ticks.", ticks);
}

fn main() -> Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let rules: Vec<(char, char)> = buf.split("\n")
        .filter(|l| l.trim().len() > 0)
        .map(|l| l.trim().split(" ").collect::<Vec<&str>>())
        .map(|s| (s[1].chars().next().unwrap(), s[7].chars().next().unwrap()))
        .collect();
    part1(&rules);
    part2(&rules, 5, 60);
    Ok(())
}
