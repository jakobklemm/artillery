static GRAVITY: f64 = 9.80665;

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

struct Object {
    position: Position,
    weapons: Weapon,
}

impl Object {
    fn new(x: u32, y: u32, alt: u32) -> Self {
        let pos: Position = Position::new(x, y, alt);
        Self {
            position: pos,
            weapons: Weapon::None,
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
        }
    }

    fn mode(&self, distance: f64) -> Option<Mode> {
        let modes = self.unwrap();
        for (i, m) in modes.iter().enumerate() {
            if i >= 1 && distance >= m.3 .1 {
                Some(m)
            } else {
                None
            }
        }
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
    fn new(x: u32, y: u32, alt: u32) -> Self {
        Self { x, y, alt }
    }

    fn distance(&self, o: Self) -> f64 {
        let a = o.x - self.x;
        let b = o.y - self.y;
        let distance = a.pow(2) + b.pow(2);
        10.0 * (distance as f64).sqrt()
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
}

fn angle(v: f64, x: f64, y: f64) -> f64 {
    let sq: f64 = v.powi(4) - GRAVITY * (GRAVITY * x.powi(2) + 2.0 * y * v.powi(2));
    let inner = (v.powi(2) + sq.sqrt()) / (GRAVITY * x);
    return inner.atan();
}

fn time(v: f64, x: f64, a: f64) -> f64 {
    return x / (v * a.cos());
}

fn main() {
    let t = Position::new(1000, 1000, 10);
    let o = Position::new(0, 0, 0);
    let m = Mode::new("Short", 243.0, (1000, 10000));
    println!("Distance: {}", t.distance(o.clone()));
    println!("Bearing: {}", t.bearing(o.clone()));
    println!("Angle: {}", t.angle(o, m));
}
