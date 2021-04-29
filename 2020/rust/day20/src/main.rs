use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

lazy_static! {
    static ref RE_TILE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
}

type Line = Vec<char>;

#[derive(Debug, Clone, Hash, Eq)]
struct Tile {
    id: usize,
    data: Vec<Line>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

// impl Hash for Tile {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.id.hash(state);
//         self.data.hash(state);
//     }
// }

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id);
        for line in self.data.iter() {
            writeln!(f, "{}", line.iter().collect::<String>());
        }
        Ok(())
    }
}

impl Tile {
    fn get_borders(&self) -> Vec<Line> {
        vec![
            self.data.first().unwrap().iter().cloned().collect(),
            self.data.last().unwrap().iter().cloned().collect(),
            self.data
                .iter()
                .map(|l| l.first().unwrap())
                .cloned()
                .collect(),
            self.data
                .iter()
                .map(|l| l.last().unwrap())
                .cloned()
                .collect(),
        ]
    }
    fn get_flips(&self) -> Vec<Tile> {
        vec![
            Tile {
                id: self.id,
                data: self.data.iter().cloned().collect(),
            },
            Tile {
                id: self.id,
                data: self.data.iter().rev().cloned().collect(),
            },
            Tile {
                id: self.id,
                data: self
                    .data
                    .iter()
                    .map(|l| l.iter().rev().cloned().collect())
                    .collect(),
            },
            // Tile {
            //     id: self.id,
            //     data: self
            //         .data
            //         .iter()
            //         .rev()
            //         .map(|l| l.iter().rev().cloned().collect())
            //         .collect(),
            // },
        ]
    }
    fn get_rotations(&self) -> Vec<Tile> {
        let mut rotateds = vec![self.clone()];

        let mut last = self.clone();
        for _ in 0..3 {
            let mut tile_copy = last.clone();
            for y in 0..(self.data.len()) {
                for x in 0..(self.data[0].len()) {
                    tile_copy.data[y][x] = last.data[tile_copy.data[y].len() - x - 1][y]
                }
            }
            last = tile_copy.clone();
            rotateds.push(tile_copy);
        }

        rotateds
    }
    fn get_all_transformations(&self) -> HashSet<Tile> {
        self.get_flips()
            .iter()
            .flat_map(|t| t.get_rotations())
            .collect()
    }
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    let paragraphs: Vec<&str> = input.split("\n\n").collect();
    let tiles: Vec<Tile> = paragraphs
        .into_iter()
        .map(|p| {
            let mut lines = p.lines();
            let first_line = lines.next().unwrap();
            let comps = RE_TILE.captures(first_line).unwrap();
            let id = comps.get(1).unwrap().as_str().parse().unwrap();
            let data: Vec<Line> = lines.map(|l| l.chars().collect()).collect();

            Tile { id, data }
        })
        .collect();

    tiles
}

fn part_1(input: &str) {
    let tiles = parse_tiles(input);
    // println!("{:?}", tiles);
    // for tile in tiles.iter() {
    //     println!("{:?}", tile.get_borders());
    // }

    // let first_tile = tiles.first().unwrap();

    // for (i, t) in first_tile.get_all_transformations().iter().enumerate() {
    //     println!("{}", i);
    //     println!("{}", t);
    //     println!("{:?}", t.get_borders());
    // }

    let mut seens: HashMap<Line, HashSet<usize>> = HashMap::new();

    let tiles_map: HashMap<usize, Tile> = tiles.iter().map(|t| (t.id, t.clone())).collect();
    tiles
        .into_iter()
        .flat_map(|t| t.get_all_transformations())
        .for_each(|t| {
            t.get_borders().into_iter().for_each(|b| {
                if let Some(sb) = seens.get_mut(&b) {
                    sb.insert(t.id);
                } else {
                    seens.insert(
                        b.to_owned(),
                        vec![t.id].into_iter().collect::<HashSet<usize>>(),
                    );
                }
            })
        });

    // println!("{:?}", seens);
    seens = seens.into_iter().filter(|(_, v)| v.len() == 1).collect();
    println!("{:?}", seens);
    let mut possible_edges: HashMap<usize, Vec<Line>> = HashMap::new();
    seens.into_iter().for_each(|(k, v)| {
        let &id = v.iter().next().unwrap();
        if let Some(t) = possible_edges.get_mut(&id) {
            t.push(k);
        } else {
            possible_edges.insert(id, vec![k]);
        }
    });
    println!("{:?}", possible_edges);
    possible_edges = possible_edges
        .into_iter()
        // .filter(|(k, lines)| lines.len() == 2)
        .filter(|(k, lines)| {
            let tile = tiles_map.get(&k).unwrap();
            for form in tile.get_all_transformations() {
                let bs = form.get_borders();
                if lines.iter().all(|l| bs.contains(l)) {
                    return true;
                }
            }
            false
        })
        .collect();
    println!("{:?}", possible_edges);
    // println!("{:?}", possible_edges.keys().fold(1, |acc, x| acc * x));

    // let ids: HashSet<usize> = seens.into_iter().flat_map(|(_, v)| v).collect();
    // println!("{:?}", ids);

    // for t in first_tile.get_all_transformations() {
    //     println!("{}", t); // } // for tile in tiles { //     for form in tile.get_all_transformations() {
    //         for border in form.get_borders() {

    //         }
    //     }
    // }
}

fn main() {
    let input = include_str!("../input");
    let input = include_str!("../example");
    part_1(&input);
    println!("Hello, world!");
}
