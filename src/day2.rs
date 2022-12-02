use std::collections::BinaryHeap;
use std::{fs::File, io, io::BufRead, path::Path};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Play {
    R,
    S,
    P,
}

impl Play {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Play::R,
            "B" | "Y" => Play::S,
            "C" | "Z" => Play::P,
            _ => panic!("don't know letter"),
        }
    }
}

fn score(t: Play, o: Play) -> u32 {
    let piece_score = match o {
        Play::R => 1,
        Play::S => 2,
        Play::P => 3,
    };

    let play_score = match (t, o) {
        _ if t == o => 3,
        (Play::R, Play::P) | (Play::S, Play::R) | (Play::P, Play::S) => 6,
        _ => 0,
    };

    piece_score + play_score
}

fn parse(path: &str) -> io::Result<Vec<(Play, Play)>> {
    let mut plays = Vec::new();

    for line in read_lines(path)? {
        let line = line?;
        let (t, o) = line.split_once(' ').unwrap();
        plays.push((Play::from(t), Play::from(o)))
    }

    Ok(plays)
}

fn p1(plays: Vec<(Play, Play)>) -> u32 {
    plays.iter().map(|(p1, p2)| score(*p1, *p2)).sum()
}

fn p2(elves: Vec<Vec<u32>>) -> u32 {
    let mut heap: BinaryHeap<u32> = elves.iter().map(|e| e.iter().sum()).collect();

    (0..3).into_iter().map(|_| heap.pop().unwrap()).sum()
}

pub fn go(path: &str) {
    let plays = parse(path).unwrap();

    dbg!(p1(plays));
}
