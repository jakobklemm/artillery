static GRAVITY: f64 = 9.80665;
use std::env;
use std::error::Error;
use std::fmt::Error as FmtError;
use std::io::{self, Write};

#[derive(Clone, Debug)]
enum Weapon {
    Scorcher(Vec<Mode>),
    Mortar(Vec<Mode>),
    MLRS(Vec<Mode>),
    None,
}

#[derive(Clone, Debug)]
struct Mode {
    name: String,
    vel: f64,
    range: (u32, u32),
}

#[derive(Clone, Debug)]
struct Position {
    x: u32,
    y: u32,
    alt: u32,
}

#[derive(Clone, Debug)]
struct Player {
    position: Position,
    weapon: Weapon,
}

#[derive(Clone, Debug)]
struct Action {
    origin: Player,
    target: Player,
    mode: Mode,
    distance: f64,
    alt: f64,
    time: f64,
    angle: f64,
    bearing: f64,
}

impl Player {
    fn new(x: u32, y: u32, alt: u32) -> Self {
        let p = Position::create(x, y, alt);
        Self {
            position: p,
            weapon: Weapon::None,
        }
    }

    fn from_grid(grid: String, alt: u32) -> Result<Self, Box<dyn Error>> {
        let p = Position::from_grid(grid, alt)?;
        return Ok(Self {
            position: p,
            weapon: Weapon::None,
        });
    }

    fn arm(&mut self, w: Weapon) {
        self.weapon = w;
    }

    fn execute(&self, o: Self) -> Option<Action> {
        let distance = self.position.distance(&o.position);
        let alt = self.position.alt(&o.position);
        match o.weapon.mode(distance) {
            Some(mode) => {
                let angle = self.position.angle(&o.position, mode.clone());
                let bearing = self.position.bearing(&o.position);
                let time = self.position.time(&o.position, mode.clone());
                Some(Action {
                    origin: o.clone(),
                    target: self.clone(),
                    mode: mode,
                    distance: distance,
                    alt: alt,
                    time: time,
                    angle: angle,
                    bearing: bearing,
                })
            }
            None => None,
        }
    }
}

impl Weapon {
    fn new(name: &str) -> Self {
        match name {
            "Scorcher" => {
                let modes = vec![
                    Mode::new("Short", 153.9, (826, 2415)),
                    Mode::new("Medium", 243.0, (2059, 6021)),
                    Mode::new("Far", 388.8, (5271, 15414)),
                    Mode::new("Further", 648.0, (14644, 42818)),
                    Mode::new("Extreme", 810.0, (22881, 66903)),
                ];
                return Self::Scorcher(modes);
            }
            "Mortar" => {
                let modes = vec![
                    Mode::new("Short", 70.0, (34, 499)),
                    Mode::new("Medium", 140.0, (139, 1998)),
                    Mode::new("Far", 200.0, (284, 4078)),
                ];
                return Self::Mortar(modes);
            }
            "MLRS" => {
                let modes = vec![
                    Mode::new("Short", 212.5, (799, 4604)),
                    Mode::new("Medium", 425.0, (3918, 18418)),
                    Mode::new("Far", 637.5, (7196, 41442)),
                    Mode::new("Full", 772.5, (12793, 73674)),
                ];
                return Self::MLRS(modes);
            }
            _ => return Self::None,
        }
    }

    fn unwrap(&self) -> Vec<Mode> {
        match self {
            Self::Scorcher(modes) => {
                return modes.to_vec();
            }
            Self::Mortar(modes) => {
                return modes.to_vec();
            }
            Self::MLRS(modes) => {
                return modes.to_vec();
            }
            Self::None => return Vec::new(),
        }
    }

    fn mode(&self, distance: f64) -> Option<Mode> {
        let modes = self.unwrap();
        for (_i, m) in modes.iter().enumerate() {
            let (min, max) = m.range;
            if distance >= min as f64 && distance < max as f64 {
                return Some(m.clone());
            } else {
                continue;
            }
        }
        return None;
    }
}

impl Mode {
    fn new(name: &str, vel: f64, range: (u32, u32)) -> Self {
        Self {
            name: name.to_string(),
            vel,
            range,
        }
    }
}

impl Position {
    fn create(x: u32, y: u32, alt: u32) -> Self {
        Self { x, y, alt }
    }

    fn from_grid(grid: String, alt: u32) -> Result<Self, Box<dyn Error>> {
        if grid.len() == 8 {
            let (x, y) = grid.split_at(4);
            let ux: u32 = x.parse::<u32>()?;
            let uy: u32 = y.parse::<u32>()?;
            return Ok(Self {
                x: 10 * ux,
                y: 10 * uy,
                alt,
            });
        } else {
            Err(Box::new(FmtError))
        }
    }

    fn distance(&self, o: &Self) -> f64 {
        let a = (o.x as i64) - (self.x as i64);
        let b = (o.y as i64) - (self.y as i64);
        let distance = a.pow(2) + b.pow(2);
        (distance as f64).sqrt()
    }

    fn alt(&self, o: &Self) -> f64 {
        (self.alt as f64) - o.alt as f64
    }

    fn bearing(&self, o: &Self) -> f64 {
        let a = ((o.x as i64) - (self.x as i64)) as f64;
        let b = ((o.y as i64) - (self.y as i64)) as f64;
        90.0 - (b / a).atan()
    }

    fn angle(&self, o: &Self, m: Mode) -> f64 {
        let x = self.distance(o);
        let y = self.alt(o);
        let sq: f64 =
            m.vel.powi(4) - GRAVITY * (GRAVITY * m.vel.powi(2) + 2.0 * y as f64 * m.vel.powi(2));
        let inner = (m.vel.powi(2) + sq.sqrt()) / (GRAVITY * x);
        return inner.atan();
    }

    fn time(&self, o: &Self, m: Mode) -> f64 {
        return self.distance(o) / (m.vel * (self.angle(o, m)).clone());
    }
}

use std::f64::consts::PI;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pos = &args[1];
    let alt = &args[2];
    let alt: u32 = alt.parse::<u32>().expect("Invalid arguments");
    let mut player = Player::from_grid(pos.to_string(), alt).expect("Position not valid!");
    let weapon = Weapon::new("Scorcher");
    player.arm(weapon);
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_line) => {
                let mut s: Vec<&str> = input.split_whitespace().collect();
                if s.len() != 2 {
                    println!("Not enough arguments!");
                    continue;
                }
                let pos = &s[0];
                let alt = &s[1];
                let alt: u32 = alt.parse::<u32>().expect("Invalid alt");
                let enemy = Player::from_grid(pos.to_string(), alt).unwrap();
                match enemy.execute(player.clone()) {
                    Some(a) => {
                        let angle = 90.0 - ((a.angle / PI) * 180.0);
                        println!("Mode: {}, Angle: {}", a.mode.name, angle);
                        println!("Distance: {}, Time: {}", a.distance, a.time);
                    }
                    None => println!("Invalid!"),
                }
            }
            Err(err) => println!("error: {}", err),
        }
    }
}

#[test]
fn test_grid() {
    let target = Player::from_grid("11112222".to_string(), 42).unwrap();
    assert_eq!(target.position.x, Player::new(11110, 22220, 42).position.x)
}

#[test]
fn test_distance() {
    let t1 = Player::from_grid("10000000".to_string(), 42).unwrap();
    let t2 = Player::from_grid("00000000".to_string(), 42).unwrap();
    let d = t1.position.distance(&t2.position);
    assert_eq!(d, 10000.0);
}

#[test]
fn test_scorcher() {
    let mut t1 = Player::from_grid("10000000".to_string(), 42).unwrap();
    let weapon = Weapon::new("Scorcher");
    t1.arm(weapon);
    assert_eq!(t1.weapon.unwrap().len(), 5);
}

#[test]
fn test_mode() {
    let mut t1 = Player::from_grid("02000000".to_string(), 42).unwrap();
    let t2 = Player::from_grid("00000000".to_string(), 42).unwrap();
    let weapon = Weapon::new("Mortar");
    t1.arm(weapon);
    let mode = t1.weapon.mode(t1.position.distance(&t2.position)).unwrap();
    assert_eq!(mode.name, "Far");
}

#[test]
fn test_action() {
    let mut t1 = Player::from_grid("02000000".to_string(), 42).unwrap();
    let t2 = Player::from_grid("00000000".to_string(), 42).unwrap();
    let weapon = Weapon::new("Mortar");
    t1.arm(weapon);
    let a = t2.execute(t1).unwrap();
    assert!(a.angle.to_degrees() >= 76.0);
}
