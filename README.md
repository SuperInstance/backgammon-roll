# Backgammon Roll

**A Rust library for backgammon dice rolling and move generation**, implementing the standard rules for converting dice rolls into legal move sequences.

## Why It Matters

Backgammon is one of the oldest board games still played — 5,000 years old. It's also a canonical testbed for game AI and probability analysis. The game's unique twist: each turn, players roll two dice and must distribute the values as moves across the board. Doubles (matching dice) grant four moves instead of two, creating combinatorial complexity.

This crate models the core mechanics:

- **Dice rolling** with a deterministic PRNG seeded from system time
- **Move enumeration** — converting `(Die, Die)` into a list of moves (2 for non-doubles, 4 for doubles)
- **Board representation** — the 24-point backgammon board with bar and bear-off areas

## How It Works

**Dice**: Two six-sided dice are rolled using a multiplicative congruential PRNG (`x = x * constant >> shift`). This isn't cryptographically secure but is sufficient for game simulation. Each die produces a value 1–6.

**Move generation**: `moves_from(d1, d2)` returns the list of pip values to play. For a non-double like (3, 5), it returns `[3, 5]` — two moves of 3 and 5 pips. For doubles like (4, 4), it returns `[4, 4, 4, 4]` — four moves of 4 pips each.

**Board representation**: The board uses the standard signed-integer encoding: positive values at each point represent White checkers, negative represent Black. The starting position has checkers at the canonical backgammon layout: 2 on point 1, 5 on point 12, 3 on point 17, 5 on point 19 (and mirror for Black).

## Quick Start

```rust
use backgammon_roll::{roll_dice, moves_from, Die, Board};

let (d1, d2) = roll_dice();
println!("Rolled: {} {}", d1.0, d2.0);

// Non-doubles give 2 moves
let moves = moves_from(Die(3), Die(5));
assert_eq!(moves, vec![3, 5]);

// Doubles give 4 moves
let doubles = moves_from(Die(4), Die(4));
assert_eq!(doubles, vec![4, 4, 4, 4]);

let board = Board::starting();
println!("Point 1 has {} white checkers", board.at(0));
```

## API

- **`Die(u8)`** — A single die value (1–6)
- **`Checker`** — Enum: `White`, `Black`
- **`Board`** — 24-point board with bar and bear-off tracking
  - `starting()` — Standard backgammon starting position
  - `at(idx)` — Check the count/sign at a point
- **`roll_dice()`** → `(Die, Die)` — Roll two dice
- **`moves_from(d1, d2)`** → `Vec<u8>` — Enumerate legal move pips

## Architecture Notes

Provides the game-mechanics layer for SuperInstance game-engine experiments. The board representation is designed to be extended with full move validation, pip counting, and AI evaluation. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
