use std::{fs::{File}, path::Path, io::BufRead, io};
use std::collections::BinaryHeap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse(path: &str) -> io::Result<Vec<Vec<u32>>> {
    let mut elves = Vec::from([Vec::new()]);

    for line in read_lines(path)? {
        match line?.as_ref() {
            "" => elves.push(Vec::new()),
            s => {
                let calories: u32 = s.parse().unwrap();
                elves.last_mut().unwrap().push(calories);
            }
        }

    }

    Ok(elves)
}

fn p1(elves: Vec<Vec<u32>>) -> u32 {
    elves.iter().map(|e| e.iter().sum()).max().unwrap()
}

fn p2(elves: Vec<Vec<u32>>) -> u32 {
    let mut heap: BinaryHeap<u32> = elves.iter().map(|e| e.iter().sum()).collect();

    (0..3).into_iter().map(|_| heap.pop().unwrap()).sum()
}

pub fn go(path: &str) {
    let elves = parse(path).unwrap();

    dbg!(p2(elves));
}
