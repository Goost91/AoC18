use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use core::borrow::Borrow;
use std::collections::HashSet;

pub fn part1() {
    let input = BufReader::new(File::open("input/day01a").unwrap()).lines();
    let result = input.into_iter().fold(0, |sum, val| { sum + val.unwrap().parse::<isize>().unwrap() });
    println!("{}", result);
}

pub fn part2() {
    let input = BufReader::new(File::open("input/day01a").unwrap()).lines();
    let values: Vec<String> = input.into_iter().map(|a| a.unwrap()).collect();

    let mut values_reached = HashSet::new();
    let mut acc = 0;

    loop {
        for line in &values {
            if values_reached.contains(acc.borrow()) {
                println!("{} reached twice first", acc);
                return;
            } else {
                values_reached.insert(acc);
            }
            acc += line.parse::<i32>().unwrap();
        }
    }
}