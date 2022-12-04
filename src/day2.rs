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
    P,
    S,
}

impl Play {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Play::R,
            "B" | "Y" => Play::P,
            "C" | "Z" => Play::S,
            _ => panic!("don't know letter"),
        }
    }
}

fn score(t: Play, o: Play) -> u32 {
    let piece_score = match o {
        Play::R => 1,
        Play::P => 2,
        Play::S => 3,
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

fn p2(plays: Vec<(Play, Play)>) -> u32 {
    plays
        .iter()
        .map(|(p1, play)| {
            let p2 = match (p1, play) {
                (Play::P, Play::R) | (Play::R, Play::P) | (Play::S, Play::S) => Play::R,
                (Play::R, Play::R) | (Play::S, Play::P) | (Play::P, Play::S) => Play::S,
                (Play::S, Play::R) | (Play::P, Play::P) | (Play::R, Play::S) => Play::P,
            };
            score(*p1, p2)
        })
        .sum()
}

pub fn go(path: &str) {
    let plays = parse(path).unwrap();

    dbg!(p2(plays));
}
