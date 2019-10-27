use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut tried_already = HashSet::new();
    match start_bucket {
        Bucket::One => tried_already.insert((0, capacity_2)),
        Bucket::Two => tried_already.insert((capacity_1, 0)),
    };

    let mut queue = VecDeque::from(vec![(0, 0, 0)]);

    while let Some((moves, bucket_1, bucket_2)) = queue.pop_front() {
        if tried_already.contains(&(bucket_1, bucket_2)) {
            continue;
        }
        tried_already.insert((bucket_1, bucket_2));
        if bucket_1 == goal {
            return Some(BucketStats {
                moves: moves,
                goal_bucket: Bucket::One,
                other_bucket: bucket_2,
            });
        } else if bucket_2 == goal {
            return Some(BucketStats {
                moves: moves,
                goal_bucket: Bucket::Two,
                other_bucket: bucket_1,
            });
        }
        queue.push_back((moves + 1, 0, bucket_2));
        queue.push_back((moves + 1, bucket_1, 0));
        queue.push_back((moves + 1, capacity_1, bucket_2));
        queue.push_back((moves + 1, bucket_1, capacity_2));
        let volume = bucket_1 + bucket_2;
        if volume > capacity_2 {
            queue.push_back((moves + 1, volume - capacity_2, capacity_2));
        } else {
            queue.push_back((moves + 1, 0, volume));
        }
        if volume > capacity_1 {
            queue.push_back((moves + 1, capacity_1, volume - capacity_1));
        } else {
            queue.push_back((moves + 1, volume, 0));
        }
    }
    None
}
