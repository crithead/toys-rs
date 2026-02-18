//! Example program for a Trollway turn queue.

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Ord, Eq, PartialEq)]
struct Turn {
    tick: u32,
    id: u32,
}

// The BinaryHeap (aka Priority Queue) orders its elements from highest
// priority to lowest.  To process turns from earliest time to latest time,
// the "Ordering" of PartialOrd must be reversed.  So now BinaryHeap orders
// Turns from lowest tick (highest priority) to highest tick (lowest priority).
impl PartialOrd for Turn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.tick < other.tick {
            Some(Ordering::Greater)
        } else if self.tick > other.tick {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn main() {
    println!("Trollway turn queue example");

    const MAX_TURNS: u32 = 100;
    let mut clock: u32 = 0;
    let mut turns: BinaryHeap<Turn> = BinaryHeap::new();

    let t = clock + rand::rng().random_range(1..=20);
    turns.push(Turn { tick: t, id: 11 });
    let t = clock + rand::rng().random_range(1..=20);
    turns.push(Turn { tick: t, id: 22 });
    let t = clock + rand::rng().random_range(1..=20);
    turns.push(Turn { tick: t, id: 33 });
    let t = clock + rand::rng().random_range(1..=20);
    turns.push(Turn { tick: t, id: 44 });

    while turns.len() > 0 {
        clock += 1;
        //println!("Clock {}", clock);

        loop {
            // Process all turns at (or before) this clock tick
            if let Some(t) = turns.peek() {
                if t.tick > clock {
                    break;
                }
                match turns.pop() {
                    Some(t) => {
                        // Take ID's turn
                        take_turn(clock, &t);
                        // Schedule ID's next turn
                        turns.push(next_turn(clock, t.id));
                    }
                    None => {}
                }
                // Alternatively
                //  if let Some(t) = turns.pop() {
                //      take_turn(t);
                //      turns.push(next_turn(clock, t.id));
                //  }
            } else {
                break;
            }
        }

        if clock > MAX_TURNS {
            break;
        }
    }

    println!("turns.len() -> {}", turns.len());
}

fn take_turn(clock: u32, turn: &Turn) {
    println!("On {} Player {} acts on {}", clock, turn.id, turn.tick);
}

fn next_turn(clock: u32, id: u32) -> Turn {
    let t = clock + rand::rng().random_range(5..=15);
    Turn { tick: t, id }
}
