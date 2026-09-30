//! Backgammon dice rolling and move generation.
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Checker { White, Black }

#[derive(Debug, Clone, Copy)]
pub struct Die(pub u8);

fn simple_seed() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64
}

pub fn roll_dice() -> (Die, Die) {
    let s = simple_seed();
    let a = ((s.wrapping_mul(6364136223846793005)) >> 16) % 6 + 1;
    let b = ((s.wrapping_mul(1442695040888963407)) >> 32) % 6 + 1;
    (Die(a as u8), Die(b as u8))
}

pub fn moves_from(d1: Die, d2: Die) -> Vec<u8> {
    let Die(a) = d1;
    let Die(b) = d2;
    if a == b { vec![a; 4] } else { vec![a, b] }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(pub i8); // 1..24

#[derive(Debug, Clone)]
pub struct Board {
    /// Positive = White checkers, negative = Black checkers.
    points: [i8; 24],
    bar_white: u8,
    bar_black: u8,
    off_white: u8,
    off_black: u8,
}

impl Board {
    pub fn starting() -> Self {
        let mut pts = [0i8; 24];
        pts[0] = 2;   // White on point 1
        pts[5] = -5;  // Black on point 6
        pts[7] = -3;  // Black on point 8
        pts[11] = 5;  // White on point 12
        pts[12] = -5; // Black on point 13
        pts[16] = 3;  // White on point 17
        pts[18] = 5;  // White on point 19
        pts[23] = -2; // Black on point 24
        Self { points: pts, bar_white: 0, bar_black: 0, off_white: 0, off_black: 0 }
    }
    pub fn at(&self, idx: usize) -> i8 { self.points[idx] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn doubles() {
        let mv = moves_from(Die(3), Die(3));
        assert_eq!(mv, vec![3, 3, 3, 3]);
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
