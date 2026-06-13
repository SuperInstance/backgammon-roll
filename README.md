# backgammon-roll

**Backgammon dice rolling, move generation, and board representation in Rust.**

Backgammon is one of the oldest board games in human history (~5,000 years), played on a board of 24 points with 15 checkers per player. Each turn, two six-sided dice determine how far checkers can move. Doubles (both dice showing the same value) grant four moves instead of two. The mathematical richness comes from the 36 equally-likely dice outcomes per turn and the combinatorial explosion of legal move sequences.

## Why It Matters

Backgammon is the canonical testbed for **stochastic game AI**, sitting between deterministic games (chess, Go) and pure chance games:

- **Expectiminimax** — the generalization of minimax to games with chance nodes — was developed specifically for backgammon (Michie, 1966).
- **TD-Gammon** (Tesauro, 1995) was the first major success of temporal-difference reinforcement learning, achieving world-champion level through self-play.
- **Neural network evaluation** — TD-Gammon's 40-hidden-unit network pioneered deep RL before DeepMind existed.

The game's mathematical structure:

- **State space**: ~10²⁰ reachable board positions (compared to ~10⁴⁷ for chess)
- **Branching factor**: Up to ~400 legal move sequences per turn (with doubles)
- **Chance nodes**: 36 equally-likely dice outcomes (6×6, with doubles counting once each)

This crate provides the foundational layer: dice generation, move computation, and board state.

## How It Works

### Dice Mechanics

Two six-sided dice are rolled. If both show the same value (doubles), the player gets **four moves** of that value instead of two:

> roll(3,3) → moves = [3, 3, 3, 3] (four moves of 3)  
> roll(3,5) → moves = [3, 5] (one move of 3, one of 5)

The number of distinct dice outcomes:

| Type | Outcomes | Count |
|------|----------|-------|
| Non-doubles | (1,2), (1,3), ..., (5,6) | ⁶C₂ × 2 = 30 |
| Doubles | (1,1), (2,2), ..., (6,6) | 6 |
| **Total** | | **36** |

Probability of doubles: 6/36 = 1/6 ≈ 16.7%.

### Random Number Generation

Dice are generated using a **linear congruential generator (LCG)** seeded from system nanosecond time:

> s₀ = current_time_nanos()  
> a = (s₀ × 6364136223846793005 >> 16) mod 6 + 1  
> b = (s₀ × 1442695040888963407 >> 32) mod 6 + 1

The constants 6364136223846793005 and 1442695040888963407 are from Knuth's MMIX parameters. This produces a uniform distribution over {1, ..., 6} for each die.

### Board Representation

The board is an array of 24 points, each holding a signed i8:

> points[i] > 0 → White has `points[i]` checkers on point i+1  
> points[i] < 0 → Black has |points[i]| checkers on point i+1  
> points[i] = 0 → point is empty

Standard starting position:

| Point | Checkers | Color |
|-------|----------|-------|
| 1 | 2 | White |
| 6 | 5 | Black |
| 8 | 3 | Black |
| 12 | 5 | White |
| 13 | 5 | Black |
| 17 | 3 | White |
| 19 | 5 | White |
| 24 | 2 | Black |

Additional state: bar (hit checkers), off (borne off checkers) for each color.

### Move Generation

`moves_from(d1, d2)` returns the available move distances:

> If d1 == d2: [d1, d1, d1, d1] (four moves)  
> If d1 ≠ d2: [d1, d2] (two moves)

### Complexity

| Operation | Time | Notes |
|-----------|------|-------|
| `roll_dice()` | O(1) | Two LCG evaluations |
| `moves_from(d1, d2)` | O(1) | Return 2 or 4 elements |
| `Board::starting()` | O(1) | 24-element array init |
| Full move enumeration | O(n!) worst case | n = number of moves (2 or 4) |

The move enumeration factorial comes from the combinatorial possibilities: with 4 moves (doubles) and up to 15 checkers, the number of distinct move sequences can exceed 400.

## Quick Start

```rust
use backgammon_roll::{roll_dice, moves_from, Die, Board, Checker};

// Roll dice
let (d1, d2) = roll_dice();
println!("Rolled: {} {}", d1.0, d2.0);

// Compute available moves
let moves = moves_from(d1, d2);
println!("Moves: {:?}", moves);

// Set up starting position
let board = Board::starting();
println!("Point 1: {} checkers", board.at(0));   // White: 2
println!("Point 6: {} checkers", board.at(5));   // Black: -5
println!("Point 12: {} checkers", board.at(11)); // White: 5

// Doubles give 4 moves
let quad = moves_from(Die(3), Die(3));
assert_eq!(quad, vec![3, 3, 3, 3]);
```

## API

- **`Checker`** — White, Black (enum)
- **`Die(u8)`** — Single die value (1–6)
- **`Point(i8)`** — Board position (1–24)
- **`Board`** — 24-point array + bar/off state: `starting()`, `at(idx) → i8`
- **`roll_dice() → (Die, Die)`** — LCG-seeded pseudo-random roll
- **`moves_from(d1, d2) → Vec<u8>`** — Available move distances (2 or 4 values)

## Architecture Notes

The γ+η=C identity in backgammon: γ (generative capacity) is the branching factor — the number of legal move sequences per turn, typically 20–400. η (evaluative depth) is the evaluation function's ability to distinguish good positions from bad. C = playing strength. TD-Gammon achieved world-class C by maximizing both: deep search (η) over the full move tree (γ). The dice add stochasticity that makes pure γ maximization insufficient — you must also handle the 36-outcome expectation at each chance node.

## References

1. Tesauro, G. (1995). "Temporal Difference Learning and TD-Gammon." *Communications of the ACM*, 38(3). — TD-learning applied to backgammon.
2. Michie, D. (1966). "Game Playing and Game Learning Automata." *Advances in Programming and Non-Numerical Computation*. — First expectiminimax.
3. Russell, S. & Norvig, P. (2020). *AI: A Modern Approach* (4th ed.), §5.5. — Expectiminimax and stochastic games.
4. Hoyle, E. (1742). *A Short Treatise on the Game of Back-Gammon*. — Historical rules.

## License

MIT
