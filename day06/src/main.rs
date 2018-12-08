use std::cmp;
use std::error::Error;
use std::io::{self, Read};
use std::collections::{HashMap, HashSet};


type Result<T> = std::result::Result<T, Box<Error>>;
type Point = (u32, u32);


struct Grid {
    width: u32,
    height: u32,
    centers: Vec<Point>,
}

impl Grid {
    fn new(center_points: &[(u32, u32)]) -> Grid {
        let x0 = center_points.iter().map(|(x, _)| x).min().unwrap();
        let y0 = center_points.iter().map(|(_, y)| y).min().unwrap();
        Grid {
            width: center_points.iter().map(|(x, _)| x).max().unwrap() + 1,
            height: center_points.iter().map(|(_, y)| y).max().unwrap() + 1,
            centers: center_points.iter().map(|(x, y)| (x - x0, y - y0)).collect(),
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn iter_points<'a>(&'a self) -> impl Iterator<Item=Point> + 'a {
        (0..self.width)
            .flat_map(move |x| std::iter::repeat(x).zip(0..self.height))
    }

    fn dist_to_centers(&self, pt: &Point) -> u32 {
        self.centers.iter().map(|ct| dist(pt, ct)).sum()
    }

    fn is_finite(&self, pts: &HashSet<Point>) -> bool {
        pts.iter()
            .all(|(x, y)| (*x > 0) &&
                          (*y > 0) &&
                          (*x < (self.width - 1)) &&
                          (*y < (self.height -1)))
    }
}


fn dist((ax, ay): &Point, (bx, by): &Point) -> u32 {
    (cmp::max(ax, bx) - cmp::min(ax, bx) +
     cmp::max(ay, by) - cmp::min(ay, by))
}


fn part1(center_points: &[(u32, u32)]) {
    let grid = Grid::new(center_points);
    let mut point_owners: HashMap<Point, char> = HashMap::new();
    let mut owned_points: HashMap<char, HashSet<Point>> = HashMap::new();

    for (idx, pt) in grid.centers.iter().enumerate() {
        let id = std::char::from_u32(idx as u32 + '😀' as u32).unwrap();
        point_owners.insert(*pt, id);
        owned_points.insert(id, HashSet::new());
        owned_points.get_mut(&id).unwrap().insert(*pt);
    }
    for pt in grid.iter_points() {
        let mut owner = '-';
        let mut min_dist = grid.area();
        for cpt in &grid.centers {
            let dst = dist(&pt, cpt);
            if dst < min_dist {
                min_dist = dst;
                owner = point_owners[cpt];
            } else if dst == min_dist {
                owner = '.';
            }
        }
        if owner != '.' && !point_owners.contains_key(&pt) {
            owned_points.entry(owner).or_insert(HashSet::new()).insert(pt);
        }
        point_owners.insert(pt, owner);
    }
    let largest_area = owned_points.values()
        .filter(|&pts| grid.is_finite(pts))
        .map(|pts| pts.len())
        .max().unwrap();
    println!("Largest area: {}", largest_area);
}

fn part2(center_points: &[(u32, u32)]) {
    let grid = Grid::new(center_points);
    let size = grid.iter_points()
        .filter(|pt| grid.dist_to_centers(pt) < 10000)
        .count();
    println!("Size of region: {}", size);
}


fn main() -> Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    let points: Vec<(u32, u32)> = buf.split("\n")
        .filter(|l| l.trim().len() > 0)
        .map(|l| l.trim().split(", ").collect::<Vec<&str>>())
        .map(|s| (s[0], s[1]))
        .map(|(x, y)| Ok((x.parse::<u32>()?, y.parse::<u32>()?)))
        .collect::<Result<_>>()?;
    part1(&points);
    part2(&points);
    Ok(())
}
