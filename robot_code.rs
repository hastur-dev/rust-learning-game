// Rust Robot Programming - External File Mode
// Save this file and the game will automatically detect changes!
// Use your favorite IDE/editor to write code here.

// Level 2 Strategy: Find and grab the scanner item
// Note: You must grab ALL items before you can complete the level!

// Enable auto-grab to automatically collect items when moving
// Standalone advanced grid coverage with random start nudge + BFS path stitching.
// Build: `cargo new adv_search && cd adv_search && replace main.rs`
// Run:   `cargo run`

use std::collections::{HashSet, HashMap, VecDeque};
use rand::{rngs::StdRng, SeedableRng, Rng};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos { x: i32, y: i32 }

#[derive(Clone, Debug)]
struct Grid {
    w: i32,
    h: i32,
    blockers: HashSet<Pos>, // cells that are blocked
}

impl Grid {
    fn in_bounds(&self, p: Pos) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.w && p.y < self.h
    }
    fn is_free(&self, p: Pos) -> bool {
        self.in_bounds(p) && !self.blockers.contains(&p)
    }
}

// Return 4-neighborhood
fn n4(p: Pos) -> [Pos; 4] {
    [
        Pos { x: p.x + 1, y: p.y },
        Pos { x: p.x - 1, y: p.y },
        Pos { x: p.x,     y: p.y + 1 },
        Pos { x: p.x,     y: p.y - 1 },
    ]
}

// BFS from `start` to ANY of the `targets`; returns path of positions (excluding start, including goal).
fn bfs_to_any(grid: &Grid, start: Pos, targets: &HashSet<Pos>) -> Option<Vec<Pos>> {
    if targets.is_empty() { return None; }
    let mut q = VecDeque::new();
    let mut seen: HashSet<Pos> = HashSet::new();
    let mut prev: HashMap<Pos, Pos> = HashMap::new();

    q.push_back(start);
    seen.insert(start);

    while let Some(u) = q.pop_front() {
        if u != start && targets.contains(&u) {
            // reconstruct path
            let mut path = vec![u];
            let mut cur = u;
            while cur != start {
                cur = *prev.get(&cur).unwrap();
                path.push(cur);
            }
            path.pop(); // remove start
            path.reverse();
            return Some(path);
        }
        for v in n4(u) {
            if grid.is_free(v) && seen.insert(v) {
                prev.insert(v, u);
                q.push_back(v);
            }
        }
    }
    None
}

// Fisher–Yates over 4 dirs with a small RNG, to randomize the initial nudge each run.
fn shuffled_dirs(rng: &mut StdRng) -> [(i32,i32); 4] {
    let mut dirs = [(1,0), (-1,0), (0,1), (0,-1)];
    for i in (1..dirs.len()).rev() {
        let j = rng.gen_range(0..=i);
        dirs.swap(i, j);
    }
    dirs
}

// Advanced coverage planner:
// 1) random nudge (one step) if possible
// 2) build set of free cells
// 3) repeatedly BFS to nearest uncovered free cell, stitch paths
// Returns a list of unit steps (dx,dy) you can replay.
fn advanced_search(grid: &Grid, start: Pos, rng_seed: u64) -> Vec<(i32,i32)> {
    let mut rng = StdRng::seed_from_u64(rng_seed);
    let mut path_moves: Vec<(i32,i32)> = Vec::new();
    let mut cur = start;

    // 0) random nudge
    for (dx,dy) in shuffled_dirs(&mut rng) {
        let np = Pos { x: cur.x + dx, y: cur.y + dy };
        if grid.is_free(np) {
            path_moves.push((dx,dy));
            cur = np;
            break;
        }
    }

    // 1) remaining free cells to cover (excluding where we stand)
    let mut remaining: HashSet<Pos> = (0..grid.w)
        .flat_map(|x| (0..grid.h).map(move |y| Pos { x, y }))
        .filter(|&p| grid.is_free(p))
        .collect();
    remaining.remove(&cur);

    // 2) greedy nearest-frontier coverage (BFS routing)
    // safety cap in case of bugs
    let step_budget = (grid.w as usize * grid.h as usize) * 16;
    let mut steps_done = 0usize;

    while !remaining.is_empty() && steps_done < step_budget {
        if let Some(path_to_target) = bfs_to_any(grid, cur, &remaining) {
            for nxt in path_to_target {
                let dx = (nxt.x - cur.x).signum();
                let dy = (nxt.y - cur.y).signum();
                debug_assert!(dx.abs() + dy.abs() == 1, "non-unit step in path");

                path_moves.push((dx,dy));
                cur = nxt;
                remaining.remove(&cur);
                steps_done += 1;
                if steps_done >= step_budget { break; }
            }
        } else {
            // disconnected remaining due to blockers boxing areas off
            break;
        }
    }

    path_moves
}

// -------------- Demo --------------
fn main() {
    // Example grid: 16x10 with some blockers. Adjust to match your level.
    let w = 16;
    let h = 10;
    let start = Pos { x: 1, y: 1 };

    let mut blockers: HashSet<Pos> = HashSet::new();
    // sprinkle a few example blockers (replace with your layout):
    for x in 5..10 {
        blockers.insert(Pos { x, y: 4 });
    }
    blockers.insert(Pos { x: 3, y: 2 });
    blockers.insert(Pos { x: 3, y: 3 });
    blockers.insert(Pos { x: 12, y: 7 });

    let grid = Grid { w, h, blockers };

    // seed: change this value or re-run to get a different starting nudge
    let seed = 0xC0FFEEu64 ^ (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64);

    let moves = advanced_search(&grid, start, seed);

    // Print the plan and also as human-friendly commands
    println!("Total steps: {}", moves.len());
    for (i, (dx,dy)) in moves.iter().enumerate() {
        let name = match (dx,dy) {
            ( 1, 0) => "move(right)",
            (-1, 0) => "move(left)",
            ( 0, 1) => "move(down)",
            ( 0,-1) => "move(up)",
            _        => "/* invalid step */",
        };
        println!("{:4}: {}   // dx={}, dy={}", i+1, name, dx, dy);
    }
}


// Try this function to search all reachable areas:
// search_all();
// Once you find the scanner item (marked with "!"), grab it:
// grab();  // This will unlock the scan() function!

// You can also use:
// move(right);
// move(up);
// grab();  // Available from Level 2+ - REQUIRED to pick up scanner!
// scan(left);  // Available ONLY after grabbing scanner with grab()

// IMPORTANT: Items and obstacles ("!" and "?") only appear on explored squares!

// Example: Manual exploration to find items
// set_auto_grab(true);
// move(right);
// move(down);
// move(left);
// move(up);

// Example: Manual grabbing
// set_auto_grab(false);
// grab();

// Example: Advanced exploration with auto-grab
// set_auto_grab(true);
// search_all();
// move(right);
// move(right);
// set_auto_grab(false);  // Disable for precise control
