//! Kingsroad - a dice race in Rust.
//!
//! You and a rival rider race down a 60-space road. Roll the die on
//! your turn; the rival rolls right after you. Tailwind spaces fling
//! you forward, mud drags you back, and you must land exactly on
//! space 60 - overshoot bounces you back by the excess.
//! Commands: 'roll' (or just Enter), 'quit'. Usage: kingsroad [--seed N]

use std::io::{self, BufRead};

const TRACK: i32 = 60;
const TAILWIND: [i32; 3] = [12, 30, 48];
const MUD: [i32; 3] = [7, 23, 41];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn die(&mut self) -> i32 {
        (self.next() % 6) as i32 + 1
    }
}

fn apply_space(pos: i32, who: &str) -> i32 {
    if TAILWIND.contains(&pos) {
        println!("  {who} caught a tailwind! +4 spaces.");
        pos + 4
    } else if MUD.contains(&pos) {
        println!("  {who} sank into the mud. -3 spaces.");
        (pos - 3).max(0)
    } else {
        pos
    }
}

fn take_turn(pos: i32, who: &str, rng: &mut Rng) -> i32 {
    let roll = rng.die();
    let mut next = pos + roll;
    if next > TRACK {
        next = TRACK - (next - TRACK);
        println!("  {who} rolled {roll} - overshoot! Bounces back to {next}.");
    } else {
        println!("  {who} rolled {roll} -> space {next}.");
    }
    apply_space(next, who)
}

fn main() {
    let mut seed: u64 = 20261125;
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--seed" && i + 1 < args.len() {
            seed = args[i + 1].parse().unwrap_or(seed);
            i += 1;
        }
        i += 1;
    }
    let mut rng = Rng(if seed == 0 { 1 } else { seed });

    println!("=== KINGSROAD ===");
    println!("Race your rival to space {TRACK}. Exact landing wins; overshoot bounces back.");
    println!("Tailwind at 12/30/48 (+4). Mud at 7/23/41 (-3). Type 'roll' or 'quit'.\n");

    let mut you = 0i32;
    let mut rival = 0i32;
    let mut round = 1;
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let winner: &str = loop {
        println!("-- Round {round} --  You: {you}   Rival: {rival}");
        print!("> ");
        use std::io::Write;
        let _ = io::stdout().flush();
        let line = match lines.next() {
            Some(Ok(l)) => l,
            _ => {
                println!();
                break "nobody";
            }
        };
        let cmd = line.trim().to_lowercase();
        if cmd == "quit" || cmd == "q" {
            break "nobody";
        }
        if cmd != "roll" && cmd != "r" && !cmd.is_empty() {
            println!("Type 'roll' or 'quit'.");
            continue;
        }
        you = take_turn(you, "You", &mut rng);
        if you == TRACK {
            break "you";
        }
        rival = take_turn(rival, "Rival", &mut rng);
        if rival == TRACK {
            break "rival";
        }
        println!();
        round += 1;
    };

    println!("\n=== GAME OVER ===");
    match winner {
        "you" => println!("You reach the end of the king's road first, in {round} rounds. The road is yours!"),
        "rival" => println!("The rival rider reaches space {TRACK} first, in {round} rounds. The road remembers."),
        _ => println!("The race is called off. You: {you}, Rival: {rival}."),
    }
}
