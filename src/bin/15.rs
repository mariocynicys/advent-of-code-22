// //! WARNING: Very complex and over engineered.
// #![allow(unused_variables, unused_imports)]
// // Requires +nightly.
// #![feature(iter_advance_by)]
// #![feature(linked_list_cursors)]

// use std::collections::{HashSet, LinkedList};

// struct Sensor {
//     sensor: (i32, i32),
//     beacon_dist: i32,
// }

// impl Sensor {
//     fn new(sensor: (i32, i32), beacon: (i32, i32)) -> Self {
//         let mut this = Self {
//             sensor,
//             beacon_dist: 0,
//         };
//         this.beacon_dist = this.manhatten(beacon);
//         this
//     }

//     fn manhatten(&self, point: (i32, i32)) -> i32 {
//         let x_diff = self.sensor.0.abs_diff(point.0);
//         let y_diff = self.sensor.1.abs_diff(point.1);
//         (x_diff + y_diff) as i32
//     }

//     /// The span @y=y where the sensor blocks the possibility of a beacon in.
//     fn coverage_at_y(&self, y: i32) -> Option<(i32, i32)> {
//         let x = self.sensor.0;
//         let diff = self.beacon_dist - self.manhatten((x, y));
//         (diff >= 0).then_some((x - diff, x + diff))
//     }
// }

// fn parse_input(input: &str) -> (Vec<Sensor>, Vec<(i32, i32)>) {
//     let mut sensors = Vec::new();
//     let mut beacons = Vec::new();
//     for line in input.trim().lines() {
//         let rest = line.trim().trim_start_matches("Sensor at");
//         let (xs, rest) = rest.trim_start_matches(" x=").split_once(',').unwrap();
//         let (ys, rest) = rest.trim_start_matches(" y=").split_once(':').unwrap();
//         let rest = rest.trim_start_matches(" closest beacon is at");
//         let (xb, rest) = rest.trim_start_matches(" x=").split_once(',').unwrap();
//         let yb = rest.trim_start_matches(" y=");
//         let (xs, ys, xb, yb) = (
//             xs.parse().unwrap(),
//             ys.parse().unwrap(),
//             xb.parse().unwrap(),
//             yb.parse().unwrap(),
//         );
//         sensors.push(Sensor::new((xs, ys), (xb, yb)));
//         beacons.push((xb, yb));
//     }
//     (sensors, beacons)
// }

// /// Returns the x coordinates for devices at y=y.
// fn devices_at_y(devices: Vec<(i32, i32)>, y: i32) -> HashSet<i32> {
//     devices
//         .into_iter()
//         .filter_map(|device| (device.1 == y).then_some(device.0))
//         .collect()
// }

// /// Checks if number is in range.
// fn in_range(range: &(i32, i32), number: i32) -> bool {
//     range.0 <= number && number <= range.1
// }

// /// Checks whether range 1 & 2 intersect or not and returns an inclusive range if they intersect.
// fn intersects(range1: &(i32, i32), range2: &(i32, i32)) -> Option<(i32, i32)> {
//     // +-1s below will account for neighboring ranges, we will compress them.
//     // remembers all ranges are inclusive.
//     (in_range(&(range1.0, range1.1 + 1), range2.0)
//         || in_range(&(range1.0 - 1, range1.1), range2.1)
//         || in_range(&(range2.0, range2.1 + 1), range1.0)
//         || in_range(&(range2.0 - 1, range2.1), range1.1))
//     .then(|| (range1.0.min(range2.0), range1.1.max(range2.1)))
// }

// trait RangeList {
//     fn add_range(&mut self, range: (i32, i32));
//     fn range_size(&self, excluding: HashSet<i32>) -> u32;
//     fn inverse_range(self, from: i32, to: i32) -> Self;
//     fn first_non_occupied(&self, occupied: HashSet<i32>) -> Option<i32>;
// }

// impl RangeList for LinkedList<(i32, i32)> {
//     fn add_range(&mut self, mut new_range: (i32, i32)) {
//         let mut cursor = self.cursor_front_mut();

//         // Delete all the ranges that intersect with the newly added range.
//         while let Some(range) = cursor.current() {
//             if let Some(sum_range) = intersects(range, &new_range) {
//                 new_range = sum_range;
//                 cursor.remove_current(); // This call auto moves the cursor.
//             } else {
//                 cursor.move_next();
//             }
//         }

//         // Move the cursor one more time to get to the beginning of the list.
//         cursor.move_next();

//         // We might not insert the new range if it's the last in the list or the list is empty.
//         let mut inserted = false;

//         // A second pass to insert the new range.
//         while let Some((start, end)) = cursor.current() {
//             if *start > new_range.1 {
//                 cursor.insert_before(new_range);
//                 inserted = true;
//                 break;
//             } else {
//                 cursor.move_next();
//             }
//         }

//         if !inserted {
//             self.push_back(new_range);
//         }
//     }

//     fn range_size(&self, excluding: HashSet<i32>) -> u32 {
//         let mut size = 0;

//         // Sort the excluded list and do a two-pointer approach since the linked list is ordered.
//         let mut excluded = Vec::from_iter(excluding);
//         excluded.sort();
//         let mut excluded_iter = excluded.iter().peekable();

//         let mut cursor = self.cursor_front();
//         while let Some(range) = cursor.current() {
//             size += range.1 - range.0 + 1;
//             while let Some(current) = excluded_iter.peek() {
//                 if in_range(range, **current) {
//                     size -= 1;
//                     excluded_iter.advance_by(1).unwrap();
//                 } else {
//                     break;
//                 }
//             }
//             cursor.move_next();
//         }
//         size as u32
//     }

//     fn inverse_range(mut self, min: i32, max: i32) -> Self {
//         // Add a beginning and ending point for the inverse range.
//         self.add_range((min - 1, min - 1));
//         self.add_range((max + 1, max + 1));
//         let mut ret = LinkedList::new();
//         let mut last_end = None;
//         for (start, end) in self {
//             // Skip ranges before `min`.
//             if end < min - 1 {
//                 continue;
//             }
//             // Skip ranges after `max`.
//             if start > max + 1 {
//                 break;
//             }
//             if let Some(last_end) = last_end {
//                 ret.push_back((last_end + 1, start - 1));
//             }
//             last_end = Some(end);
//         }
//         ret
//     }

//     // first and only!
//     fn first_non_occupied(&self, occupied: HashSet<i32>) -> Option<i32> {
//         for (start, end) in self.iter() {
//             // The point is, we know there are very very few beacons and sensors (looking at the input file).
//             // So there must be very few of these cases where a device is covering up the distress beacon spot.
//             // This also suggests that there are very few ranges in the range list (should be one range with one spot i.e. (x, x)),
//             // since there is only one valid spot for the distress beacon.
//             // So looping over the range items is actually the best way to accomplish this.
//             for x in *start..=*end {
//                 if !occupied.contains(&x) {
//                     return Some(x);
//                 }
//             }
//         }
//         None
//     }
// }

// pub fn part_one(input: &str, y: i32) -> Option<u32> {
//     let (sensors, beacons) = parse_input(input);
//     let devices_xs = devices_at_y(sensors.iter().map(|s| s.sensor).chain(beacons).collect(), y);
//     let mut range_list = LinkedList::new();

//     for range in sensors.into_iter().filter_map(|s| s.coverage_at_y(y)) {
//         range_list.add_range(range);
//     }

//     Some(range_list.range_size(devices_xs))
// }

// /// Done in a very inefficient way.
// /// Takes 850ms to compute when ran with --release. Probably tens of seconds on debug.
// pub fn part_two(input: &str, y: i32) -> Option<u64> {
//     let (sensors, beacons) = parse_input(input);
//     for iy in 0..=y {
//         let devices_xs = devices_at_y(
//             sensors
//                 .iter()
//                 .map(|s| s.sensor)
//                 .chain(beacons.iter().cloned())
//                 .collect(),
//             iy,
//         );
//         let mut range_list = LinkedList::new();
//         for range in sensors.iter().filter_map(|s| s.coverage_at_y(iy)) {
//             range_list.add_range(range);
//         }
//         let non_blocked_range = range_list.inverse_range(0, y);
//         if let Some(vacancy) = non_blocked_range.first_non_occupied(devices_xs) {
//             return Some(vacancy as u64 * 4_000_000 + iy as u64);
//         }
//     }
//     None
// }

// fn main() {
//     let input = &aoc::read_file("inputs", 15);
//     let part_one = |input: &str| part_one(input, 2_000_000);
//     aoc::solve!(1, part_one, input);
//     let part_two = |input: &str| part_two(input, 4_000_000);
//     aoc::solve!(2, part_two, input);
// }

// #[cfg(test)]
// mod t15 {
//     use super::*;

//     #[test]
//     fn test_part_one() {
//         let input = aoc::read_file("examples", 15);
//         assert_eq!(part_one(&input, 10), Some(26));
//     }

//     #[test]
//     fn test_part_two() {
//         let input = aoc::read_file("examples", 15);
//         assert_eq!(part_two(&input, 20), Some(56000011));
//     }
// }
fn main() {
    println!("
    The code for day 15 is commented out since it requires a nightly feature.
    Since it's part of the carte, I would have to add +nightly each time I run tests for another day, so commenting it out for good.
    ")
}
