use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::io::BufRead;

lazy_static! {
    static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}


pub fn part1() {
    let input = BufReader::new(File::open("input/day03a").unwrap()).lines();
    let mut map = [0u16; 1000 * 1000].to_vec();
    let mut claims: Vec<Claim> = Vec::new();

    for line in input.map(|a| a.unwrap()) {
        for cap in RE.captures_iter(line.as_str()) {
            let claim = Claim {
                id: cap[1].parse::<usize>().unwrap(),
                x: cap[2].parse::<usize>().unwrap(),
                y: cap[3].parse::<usize>().unwrap(),
                w: cap[4].parse::<usize>().unwrap(),
                h: cap[5].parse::<usize>().unwrap(),
            };

            for xx in claim.x..claim.x + claim.w {
                for yy in claim.y..claim.y + claim.h {
                    map[xx + yy * 1000usize] += 1;
                }
            }

            claims.push(claim);
        }
    }

    println!("Num of overlaps = {}", map.iter().filter(|claim| **claim > 1).count());

    for claim in claims {
        let mut claim_has_overlaps = false;
        for xx in claim.x..claim.x + claim.w {
            for yy in claim.y..claim.y + claim.h {
                if map[xx + yy * 1000usize] > 1 {
                    claim_has_overlaps = true;
                }
            }
        }

        if !claim_has_overlaps {
            println!("claim {} has no overlaps", claim.id);
        }
    }
}

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}