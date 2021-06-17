static GRAVITY: f64 = 9.80665;

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

impl Player {
    fn new(x: u32, y: u32, alt: u32) -> Self {
        let p = Position::create(x, y, alt);
        Self {
            position: p,
            weapon: Weapon::None,
        }
    }

    fn from_grid(grid: String, alt: u32) -> Option<Self> {
        match Position::from_grid(grid, alt) {
            Some(pos) => Some(Self {
                position: pos,
                weapon: Weapon::None,
            }),
            None => None,
        }
    }

    fn arm(&mut self, w: Weapon) {
        self.weapon = w;
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
                    Mode::new("Extrme", 810.0, (22881, 66903)),
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
                return Self::Mortar(modes);
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

    fn from_grid(grid: String, alt: u32) -> Option<Self> {
        if grid.len() == 8 {
            let (x, y) = grid.split_at(4);
            let ux: u32 = x.parse::<u32>().unwrap();
            let uy: u32 = y.parse::<u32>().unwrap();
            return Some(Self {
                x: 10 * ux,
                y: 10 * uy,
                alt,
            });
        } else {
            None
        }
    }

    fn distance(&self, o: Self) -> f64 {
        let a = (o.x as i64) - (self.x as i64);
        let b = (o.y as i64) - (self.y as i64);
        let distance = a.pow(2) + b.pow(2);
        (distance as f64).sqrt()
    }

    fn alt(&self, o: Self) -> u32 {
        self.alt - o.alt
    }

    fn bearing(&self, o: Self) -> f64 {
        let a = (o.x - self.x) as f64;
        let b = (o.y - self.y) as f64;
        90.0 - (b / a).atan()
    }

    fn angle(&self, o: Self, m: Mode) -> f64 {
        let x = self.distance(o.clone());
        let y = self.alt(o.clone());
        let sq: f64 =
            m.vel.powi(4) - GRAVITY * (GRAVITY * m.vel.powi(2) + 2.0 * y as f64 * m.vel.powi(2));
        let inner = (m.vel.powi(2) + sq.sqrt()) / (GRAVITY * x);
        return inner.atan();
    }

    fn time(&self, o: Self, m: Mode) -> f64 {
        return self.distance(o.clone()) / (m.vel * (self.angle(o.clone(), m)).clone());
    }
}

fn main() {
    match Player::from_grid("02950020".to_string(), 1) {
        Some(target) => {
            let mut p = Player::new(0, 0, 0);
            let h = Weapon::new("Scorcher");
            p.arm(h);
            println!("player: {}, {}", p.position.x, p.position.y);
            println!("target: {}, {}", target.position.x, target.position.y);
            let d = target.position.distance(p.position);
            println!("distance: {}", d);
            match p.weapon.mode(d) {
                Some(mode) => {
                    println!("Mode: {}, Vel: {}", mode.name, mode.vel);
                }
                None => {
                    println!("Target not in range!");
                }
            }
        }
        None => {
            println!("Inputs not valid!");
        }
    }
}
