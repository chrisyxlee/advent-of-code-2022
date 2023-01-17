use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;
use std::thread;

fn main() {
    // --snip--
    let file_path = "tmp/day15/input.txt";
    println!("In file {}", file_path);

    let lines = read_lines(file_path);
    part_1(&lines);
    //    part_2(&lines);
}

fn part_1(lines: &Vec<String>) {
    let sensors = lines
        .iter()
        .map(|line| Sensor::parse(line))
        .collect::<Vec<Sensor>>();

    let mut want_set: HashSet<i64> = HashSet::new();
    let want_y = 2000000;

    for s in &sensors {
        let y_dist = s.location.dist(Point::new(s.location.x, want_y));
        if s.dist < y_dist {
            continue;
        }

        let x_radius = s.dist - y_dist;
        for x in (s.location.x - x_radius)..(s.location.x + x_radius) {
            want_set.insert(x);
        }
    }

    println!("Part 1: {:?}", want_set.len());
}

const MULT: i64 = 4000000;

mod shared {
    use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

    pub struct Lock<T> {
        inner: Arc<RwLock<T>>,
    }

    impl<T> Lock<T> {
        pub fn new(val: T) -> Self {
            Self {
                inner: Arc::new(RwLock::new(val)),
            }
        }

        pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, T>> {
            self.inner.write()
        }

        pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
            self.inner.read()
        }

        pub fn read_only(&self) -> ReadOnly<T> {
            ReadOnly {
                inner: self.inner.clone(),
            }
        }
    }

    pub struct ReadOnly<T> {
        inner: Arc<RwLock<T>>,
    }

    impl<T> ReadOnly<T> {
        pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
            self.inner.read()
        }
    }
}

/*
fn part_2(lines: &Vec<String>) {
    let sensors = lines
        .iter()
        .map(|line| Sensor::parse(line))
        .collect::<Vec<Sensor>>();
    let borrowed_sensors = shared::Lock::new(&sensors);

    let value = Mutex::new(0);
    let num_threads = 1000;
    for i in 0..num_threads {
        let view = borrowed_sensors.read_only();
        std::thread::spawn(move || {
            let start = (MULT / num_threads) * i;
            let end = start + (MULT / num_threads);
            let num_iterations = 1000000;
            for x in start..end {
                for y in start..end {
                    let curr = Point::new(x, y);
                    let mut possible: bool = true;
                    for s in sensors {
                        if s.location.dist(curr) < s.dist || s.beacon == curr {
                            possible = false;
                            break;
                        }
                    }
                    if possible {
                        *value.lock().unwrap() = curr.tuning_frequency();
                        return;
                    }

                    if y % num_iterations == 0 {
                        if *value.lock().unwrap() > 0 {
                            return;
                        }
                        println!(
                            "{}: {}/{} = {}%",
                            i,
                            x * MULT + y,
                            MULT * MULT,
                            ((x * MULT + y) as f64) / ((MULT * MULT) as f64)
                        );
                    }
                }
            }
        });
    }
    println!("Part 2: {:?}", value.lock().unwrap());
}
*/

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x: x, y: y }
    }

    fn dist(&self, p: Point) -> i64 {
        return (self.x - p.x).abs() + (self.y - p.y).abs();
    }

    fn tuning_frequency(&self) -> i64 {
        return self.x * MULT + self.y;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sensor {
    location: Point,
    beacon: Point,
    dist: i64,
}

impl Sensor {
    pub fn parse(s: &str) -> Self {
        let sensor_re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let mut s_x = 0;
        let mut s_y = 0;
        let mut b_x = 0;
        let mut b_y = 0;

        assert!(sensor_re.is_match(s));
        for m in sensor_re.captures_iter(s) {
            for (i, capt) in m.iter().enumerate() {
                if let Some(sub) = capt {
                    if i == 0 {
                        continue;
                    }

                    let v = str2i64(sub.as_str());
                    match i {
                        1 => s_x = v,
                        2 => s_y = v,
                        3 => b_x = v,
                        4 => b_y = v,
                        _ => (),
                    }
                }
            }
        }
        let location = Point::new(s_x, s_y);
        let closest_beacon = Point::new(b_x, b_y);
        return Self {
            location: location,
            beacon: closest_beacon,
            dist: location.dist(closest_beacon),
        };
    }
}

fn str2i64(s: &str) -> i64 {
    return s.parse::<i64>().unwrap();
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    return io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect::<Vec<String>>();
}
