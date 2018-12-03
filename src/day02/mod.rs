use levenshtein::levenshtein;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use core::borrow::Borrow;
use std::collections::HashMap;

pub fn part1() {
    let input = BufReader::new(File::open("input/day02a").unwrap()).lines();

    let mut twice = 0;
    let mut thrice = 0;

    for line in input.map(|a| a.unwrap()) {
        let mut letters: HashMap<char, i32> = HashMap::new();

        line.chars()
            .into_iter()
            .for_each(|c| *letters.entry(c).or_insert(0) += 1);

        twice += if letters.iter().any(|l| *l.1 == 2) { 1 } else { 0 };
        thrice += if letters.iter().any(|l| *l.1 == 3) { 1 } else { 0 };
    }

    println!("checksum {}", twice * thrice);
}

pub fn part2() {
    let input = BufReader::new(File::open("input/day02a").unwrap()).lines();
    let mut words: Vec<String> = Vec::new();

    for line in input.map(|a| a.unwrap()) {
        for word in words.iter() {
            if levenshtein(word.as_str(), line.as_str()) == 1 {
                line.chars()
                    .zip(word.chars())
                    .into_iter()
                    .for_each(|c| {
                        if c.0 == c.1 {
                            print!("{}", c.0);
                        }
                    });
            }
        }

        words.push(line);
    }
}