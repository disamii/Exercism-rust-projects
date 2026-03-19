use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    a: u8,
    b: u8,
}

fn pour(from: u8, to: u8, cap_to: u8) -> (u8, u8) {
    let transfer = (from).min(cap_to - to);
    (from - transfer, to + transfer)
}

fn bfs(cap1: u8, cap2: u8, goal: u8, start: Bucket) -> Option<BucketStats> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    let start_state = State { a: 0, b: 0 };
    q.push_back((start_state, 0u8));
    seen.insert(start_state);

    while let Some((cur, moves)) = q.pop_front() {
        let State { a, b } = cur;

        if a == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: b,
            });
        }

        if b == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: a,
            });
        }

        let mut next = Vec::new();

        // fill
        next.push(State { a: cap1, b });
        next.push(State { a, b: cap2 });

        // empty
        next.push(State { a: 0, b });
        next.push(State { a, b: 0 });

        // pour a -> b
        let (na, nb) = pour(a, b, cap2);
        next.push(State { a: na, b: nb });

        // pour b -> a
        let (nb2, na2) = pour(b, a, cap1);
        next.push(State { a: na2, b: nb2 });

        for s in next {
            if seen.insert(s) {
                let mut next_moves = moves + 1;

                // enforce "first move must be filling start bucket"
                if moves == 0 {
                    match start {
                        Bucket::One => {
                            if !(s.a == cap1 && s.b == 0) {
                                continue;
                            }
                        }
                        Bucket::Two => {
                            if !(s.a == 0 && s.b == cap2) {
                                continue;
                            }
                        }
                    }
                }

                q.push_back((s, next_moves));
            }
        }
    }

    None
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    bfs(capacity_1, capacity_2, goal, *start_bucket)
}