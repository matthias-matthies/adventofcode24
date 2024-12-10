use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn main_01_1() {
    // Get assets/01.txt
    // Turn into two lists -> read row, split, push on vector.
    // Sort both (When using binary heap while creating the vector its already sorted)
    // pop 2 elements and get distance as absolute. add to the distance
    let mut distance = 0;
    let mut left: BinaryHeap<i32> = BinaryHeap::new();
    let mut right: BinaryHeap<i32> = BinaryHeap::new();

    let file_contents = fs::read_to_string("./assets/01.txt")
        .expect("Should have been able to read the file..");

    let rows: Vec<&str> = file_contents.split("\n").collect();

    for row in rows {
        let split: Vec<&str> = row.split("   ").collect();
        left.push(split[0].parse::<i32>().unwrap());
        right.push(split[1].parse::<i32>().unwrap());
    }

    loop {
        if left.peek() != None && right.peek() != None {
            let dist = left.pop().unwrap() - right.pop().unwrap();
            distance = distance + dist.abs();
        } else {
            break;
        }
    }

    println!("Distance: {distance}");
}

fn main_01_2() {
    // Copied lots of main_01_01
    // Changes: Put right into HashMap and keep track of count.
    // In loop calculate similarity score
    let mut similarity = 0;
    let mut left: BinaryHeap<i32> = BinaryHeap::new();
    let mut right: HashMap<i32, (i32,i32)> = HashMap::new();

    let file_contents = fs::read_to_string("./assets/01.txt")
        .expect("Should have been able to read the file..");

    let rows: Vec<&str> = file_contents.split("\n").collect();

    for row in rows {
        let split: Vec<&str> = row.split("   ").collect();
        left.push(split[0].parse::<i32>().unwrap());
        if right.contains_key(&split[1].parse::<i32>().unwrap()) {
            // Add to counter
            right.insert(split[1].parse::<i32>().unwrap(), (split[1].parse::<i32>().unwrap(), right.get(&split[1].parse::<i32>().unwrap()).unwrap().1 + 1));
        } else {
            // Insert and set counter to 0
            right.insert(split[1].parse::<i32>().unwrap(), (split[1].parse::<i32>().unwrap(), 1));
        }
    }

    loop {
        if left.peek() != None {
            let num = left.pop().unwrap();
            if right.contains_key(&num) {
                similarity = similarity + (right.get(&num).unwrap().1 * num);
            }
        } else {
            break;
        }
    }

    println!("Similarity: {similarity}");
}

struct Report02_1 {
    levels: Vec<i32>
}

enum Mode02_1 {
    INCREASE, DECREASE
}

impl Report02_1 {
    fn is_safe(&self) -> bool {
        let mut iter = Clone::clone(&self.levels).into_iter();
        let mode: Mode02_1 = if self.levels[0] < self.levels[1] {
            Mode02_1::INCREASE
        } else {
            Mode02_1::DECREASE
        };

        let mut current_num = self.levels[0];

        iter.next();
        loop {
            let next = iter.next();
            if next.is_none() {
                break;
            }
            match mode {
                Mode02_1::INCREASE => {
                    match next {
                        Some(num) => {
                            if !(num > current_num && num - current_num <= 3) {
                                return false;
                            } else {
                                current_num = num;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
                Mode02_1::DECREASE => {
                    match next {
                        Some(num) => {
                            if !(num < current_num && current_num - num <= 3) {
                                return false;
                            } else {
                                current_num = num;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }

        return true;
    }
}

fn main_02_1() {
    // Get assets/02.txt
    // Turn each line into a report struct
    // use is_safe on each struct
    // count amount

    let mut safes = 0;
    let file_contents = fs::read_to_string("assets/02.txt")
        .expect("Should be able to read file");

    let rows: Vec<&str> = file_contents.split("\n").collect();

    for row in rows {
        let levels = row.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
        let report = Report02_1 {
            levels
        };

        if report.is_safe() {
            safes = safes + 1;
        }
    }

    println!("Safes: {safes}");
}

struct Report02_2 {
    levels: Vec<i32>
}

enum Mode02_2 {
    INCREASE, DECREASE
}

impl Report02_2 {
    fn is_safe(&self) -> bool {

        let mut found_possible = false;

        for index in 0..self.levels.len() {

            if found_possible {
                return true;
            }

            let mut dampered: Vec<i32> = vec!();

            for idx in 0..self.levels.len() {
                if index != idx {
                    dampered.push(self.levels[idx])
                }
            }

            let mut iter = Clone::clone(&dampered).into_iter();
            let mode: Mode02_2 = if dampered[0] < dampered[1] {
                Mode02_2::INCREASE
            } else {
                Mode02_2::DECREASE
            };

            let mut current_num = dampered[0];

            iter.next();
            loop {
                let next = iter.next();
                if next.is_none() {
                    found_possible = true;
                    break;
                }
                match mode {
                    Mode02_2::INCREASE => {
                        match next {
                            Some(num) => {
                                if !(num > current_num && num - current_num <= 3) {
                                    break;
                                } else {
                                    current_num = num;
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                    Mode02_2::DECREASE => {
                        match next {
                            Some(num) => {
                                if !(num < current_num && current_num - num <= 3) {
                                    break;
                                } else {
                                    current_num = num;
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
            }

        }

        if found_possible {
            return true;
        }

        let mut iter = Clone::clone(&self.levels).into_iter();
        let mode: Mode02_2 = if self.levels[0] < self.levels[1] {
            Mode02_2::INCREASE
        } else {
            Mode02_2::DECREASE
        };

        let mut current_num = self.levels[0];

        iter.next();
        loop {
            let next = iter.next();
            if next.is_none() {
                break;
            }
            match mode {
                Mode02_2::INCREASE => {
                    match next {
                        Some(num) => {
                            if !(num > current_num && num - current_num <= 3) {
                                return false;
                            } else {
                                current_num = num;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
                Mode02_2::DECREASE => {
                    match next {
                        Some(num) => {
                            if !(num < current_num && current_num - num <= 3) {
                                return false;
                            } else {
                                current_num = num;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }

        return true;
    }
}

fn main_02_2() {
    // copied lots of 02_1
    // changed implementation of struct is_safe using or a greedy search
    let mut safes = 0;
    let file_contents = fs::read_to_string("assets/02.txt")
        .expect("Should be able to read file");

    let rows: Vec<&str> = file_contents.split("\n").collect();

    for row in rows {
        let levels = row.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
        let report = Report02_2 {
            levels
        };

        if report.is_safe() {
            safes = safes + 1;
        }
    }

    println!("Safes with damper: {safes}");
}

fn main_03_1() {

}
fn main_03_2() {}

fn main_04_1() {}
fn main_04_2() {}

fn main_05_1() {}
fn main_05_2() {}

fn main_06_1() {}
fn main_06_2() {}

fn main_07_1() {}
fn main_07_2() {}

fn main_08_1() {}
fn main_08_2() {}

fn main_09_1() {}
fn main_09_2() {}

fn main_10_1() {}
fn main_10_2() {}

fn main() {
    // Day 1
    println!("---");
    println!("Hello, world! Now Executing: 01");
    main_01_1();
    main_01_2();
    println!("---");
    // Day 2
    println!("Hello, world! Now Executing: 02");
    main_02_1();
    main_02_2();
    println!("---");
    // Day 3
    println!("Hello, world! Now Executing: 03");
    main_03_1();
    main_03_2();
    println!("---");
    // Day 4
    println!("Hello, world! Now Executing: 04");
    main_04_1();
    main_04_2();
    println!("---");
    // Day 5
    println!("Hello, world! Now Executing: 05");
    main_05_1();
    main_05_2();
    println!("---");
    // Day 6
    println!("Hello, world! Now Executing: 06");
    main_06_1();
    main_06_2();
    println!("---");
    // Day 7
    println!("Hello, world! Now Executing: 07");
    main_07_1();
    main_07_2();
    println!("---");
    // Day 8
    println!("Hello, world! Now Executing: 08");
    main_08_1();
    main_08_2();
    println!("---");
    // Day 9
    println!("Hello, world! Now Executing: 09");
    main_09_1();
    main_09_2();
    println!("---");
    // Day 10
    println!("Hello, world! Now Executing: 10");
    main_10_1();
    main_10_2();
    println!("---");
}
