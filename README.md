# Kingsroad (Rust)

A dice race for the terminal, written in Rust (no external crates —
standard library only).

You and a rival rider race down a 60-space road. Roll on your turn, the
rival rolls right after. Tailwind spaces (12, 30, 48) fling you 4 spaces
forward; mud (7, 23, 41) drags you 3 back. You must land exactly on
space 60 — overshoot bounces you back by the excess.

## Build

```sh
cargo build --release
```

## Play

```sh
./target/release/kingsroad              # random race
./target/release/kingsroad --seed 42    # reproducible race (fixed dice)
```

Type `roll` (or just press Enter) each round, `quit` to stop.

## Tested envelope

- Builds clean with `cargo build --release` (Rust 1.100, stable) with
  no warnings; the RNG is a small built-in xorshift64, so `--seed`
  games are fully deterministic.
- Verified end to end with scripted stdin playthroughs on `--seed 42`:
  full races reaching the game-over screen with both a player win and
  a rival win across seeds, plus a quit path.
