use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use core::borrow::Borrow;
use std::collections::HashSet;
use chrono::Utc;
use chrono::offset::TimeZone;
use chrono::DateTime;
use std::collections::HashMap;
use chrono::Timelike;
use core::borrow::BorrowMut;

pub fn parse_input() -> Vec<Event> {
    let input = BufReader::new(File::open("input/day04a").unwrap()).lines();
    let mut events : Vec<Event> = Vec::new();

    for line in input.map(|a| a.unwrap()) {
        let time = Utc.datetime_from_str(&line[1..17], "%Y-%m-%d %H:%M").unwrap();
        let mut event  = &line[19..];

        let ev = Event {
            time,
            event : event.to_string()
        };

        events.push(ev);
    }

    events.sort_by(|a,b| { a.time.cmp(&b.time)});
    return events;
}

pub fn parse_guards() -> HashMap<u32,LazyGuard> {

    let mut events : Vec<Event> = parse_input();
    let mut current_guard = 0u32;
    let mut nap_start = 0;
    let mut sleepy_guards : HashMap<u32, LazyGuard> = HashMap::new();

    for evt in &events {
        if evt.event.contains("Guard") {
            let s = evt.event[7..].to_string();
            let id = s.split(" ").collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
            current_guard = id;
        } else if evt.event.contains("falls") {
            nap_start = evt.time.minute();
        } else {
            let nap_end = evt.time.minute();
            let total = nap_end - nap_start;
            let guard = sleepy_guards.entry(current_guard).or_default();

            guard.total_sleep_time += total;
            guard.id = current_guard;

            for m in nap_start..nap_end {
                *guard.sleepy_map.entry(m).or_default() += 1;
            }
        }
    }

    return sleepy_guards;
}

pub fn part1() {
    let mut sleepy_guards = parse_guards();
    let mut most_sleep_time = 0u32;
    let mut sleepiest_guard_idx  = 0u32;

    for a in sleepy_guards.borrow() {
        if a.1.total_sleep_time > most_sleep_time {
            most_sleep_time = a.1.total_sleep_time;
            sleepiest_guard_idx = a.0.clone();
        }
    }

    let mut most_asleep_count = 0;
    let mut sleepiest_minute = 0;
    let g = sleepy_guards.entry(sleepiest_guard_idx).or_default();

    for a in &g.sleepy_map {
        if *a.1 > most_asleep_count {
            most_asleep_count = *a.1;
            sleepiest_minute = *a.0;
        }
    }

    println!("sleepiest cunt {}", &g.id * sleepiest_minute);
}

pub fn part2() {
    let mut sleepy_guards = parse_guards();

    let mut most_sleep_minute = 0u32;
    let mut most_sleep_count = 0isize;
    let mut guard : Option<LazyGuard> = None;

    for a in &sleepy_guards {
        for b in &a.1.sleepy_map {
            if *b.1 > most_sleep_count {
                most_sleep_minute = *b.0;
                most_sleep_count = *b.1;
                guard = Some(a.1.clone());
            }
        }
    }

    let mut guards = parse_guards();
    let mut lazy_guard = guard.unwrap();
    println!("guard id * sleepiest minute = {} ", (lazy_guard.id as i64) * (most_sleep_minute as i64));
}

pub struct Event {
    time: DateTime<Utc>,
    event: String
}

#[derive(Default, Clone)]
pub struct LazyGuard {
    id: u32,
    sleepy_map: HashMap<u32, isize>, // minute -> count
    total_sleep_time: u32
}