use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use core::borrow::BorrowMut;
use core::borrow::Borrow;
use std::iter::FromIterator;
use std::io::Read;
use core::mem;

pub fn part1() {
    let mut line = String::new();
    let mut input = BufReader::new(File::open("input/day05a").unwrap()).read_to_string(line.borrow_mut());

    let mut polymer = react(line.chars().collect::<Vec<char>>());

    println!("{}", polymer.len());
}

pub fn part2() {
    let mut line = String::new();
    let mut input = BufReader::new(File::open("input/day05a").unwrap()).read_to_string(line.borrow_mut());
    let mut polymer = line.chars().collect::<Vec<char>>();

    let mut best = polymer.len();

    for c in b'A'..=b'Z' {
        let clone = String::from_iter(polymer.clone()).replace(c as char, "").replace((c+32) as char, "");
        let reacted_polymer = react(clone.chars().collect());
        if reacted_polymer.len() < best {
            best = reacted_polymer.len();
        }
    }

    println!("{}", best);
}

pub fn react(mut polymer:Vec<char>) -> Vec<char> {
    let mut idx = 0;

    while idx < (polymer.len() - 1) {
        if check_reaction(polymer[idx] as u32, polymer[idx + 1] as u32) {
            polymer.remove(idx+1);
            polymer.remove(idx);

            idx -= 2;
            if idx > polymer.len()+1 {idx = 0;}
        }
        idx += 1;
    }

    polymer
}

pub fn check_reaction(a: u32, b: u32) -> bool {
    if a < b {
        b - a == 32
    } else {
        a - b == 32
    }
}