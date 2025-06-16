#[derive(Clone, Debug)]
pub struct Wheel {
    pub pins: Vec<bool>,
    pub position: usize,
}

impl Wheel {
    pub fn new(string: &str) -> Self {
        let mut pins = Vec::with_capacity(string.len());
        for c in string.chars() {
            if c == '.' {
                pins.push(false);
            } else if c == 'x' {
                pins.push(true);
            } else {
                panic!("only . and x are used for setting the pins")
            }
        }
        Self { pins, position: 0 }
    }
}

pub struct Lorenz {
    pub wheels: [Wheel; 12],
}

impl Default for Lorenz {
    fn default() -> Self {
        Self::kh_setting()
    }
}

impl Lorenz {
    pub fn kh_setting() -> Self {
        Self {
            wheels: [
                // Psi Wheels
                Wheel::new(".x...xx.x.x..xxx.x.x.xxxx.x.x.x.x.x..x.xx.x"),
                Wheel::new(".xx.x.xxx..x.x.x..x.xx.x.xxx.x....x.xx.x.x.x..x"),
                Wheel::new(".x.x.x..xxx....x.x.xx.x.x.x..xxx.x.x..x.x.xx..x.x.x"),
                Wheel::new(".xx...xxxxx.x.x.xx...x.xx.x.x..x.x.xx.x..x.x.x.x.x.x."),
                Wheel::new("xx...xx.x..x.xx.x...x.x.x.x.x.x.x.x.xx..xxxx.x.x...xx.x..x."),
                // Mu Wheels
                Wheel::new("x.x.x.x.x.x...x.x.x...x.x.x...x.x...."),
                Wheel::new(".xxxx.xxxx.xxx.xxxx.xx....xxx.xxxx.xxxx.xxxx.xxxx.xxx.xxxx..."),
                // Chi Wheels
                Wheel::new(".x...xxx.x.xxxx.x...x.x..xxx....xx.xxxx.."),
                Wheel::new("x..xxx...x.xxxx..xx..x..xx.xx.."),
                Wheel::new("..xx..x.xxx...xx...xx..xx.xx."),
                Wheel::new("xx..x..xxxx..xx.xxx....x.."),
                Wheel::new("xx..xx....xxxx.x..x.x.."),
            ],
        }
    }

    pub fn bream_setting() -> Self {
        Self {
            wheels: [
                // Psi Wheels
                Wheel::new("...xxx..xxx.xx..x.x.xx.xx.x..x..x.x.x.x.x.."),
                Wheel::new("xx.x..xxx.....xxxx.x..x.xx..xx.x.x.x.x.x.xx.x.."),
                Wheel::new("x..x..xx.xxx...xxx....xxxx.x.x.xx..x..x.x.x.x.x.x.x"),
                Wheel::new(".x....x..x.xxxxx.xx..xx..xx....x.xx.x.x.x.x.x.xx..x.x"),
                Wheel::new("x.x.x..xx..xx.xx..x...x....x.xx.xxxx.xxx..x.x...xx.x.x.x.x."),
                // Mu Wheels
                Wheel::new(".x.x.x.x.x.x.x.xxx.x.x..x.x.x.x.xxx.x"),
                Wheel::new("x....xx...xx..xx.xxxx....xx...xx.xx.x.xxxx...xx..xx..xx.x.xxx"),
                // Chi Wheels
                Wheel::new(".xxxx.x.xx.x.xx..x..xx.x....xx....xxxx..."),
                Wheel::new(".xxx....x...xx.x.x...xx.xxx..xx"),
                Wheel::new("xx..xx.xx..xxx....x..xx.xxx.."),
                Wheel::new("xxxx..x..xx..x..xx.x..xx.."),
                Wheel::new(".xxx.xxx...x..xx.x...x."),
            ],
        }
    }

    pub fn zmug_setting() -> Self {
        Self {
            wheels: [
                // Psi Wheels
                Wheel::new("xx.x..xx...xxx..xx...xx...xxxx..xxx..xxx..."),
                Wheel::new("...x...xxx..xx..xxx...xxxx...xx..xxx..xxx..x.xx"),
                Wheel::new(".x..xx..xxx..xxx..x...xxxx...x...xxx...xx...xx..xxx"),
                Wheel::new("..xxx..xx..xxx..xxxx...x...xx..xxx..x..xx...xx..xxx.x"),
                Wheel::new("x..xxx...x...xxxx..xxx..x..xxxx...xx..xxx..xx..xxx..x...xx."),
                // Mu Wheels
                Wheel::new(".x.x.xx.x.xx.xxx.xxx.xx.x.xxx.xxx.xxx"),
                Wheel::new("x.xx.x.xxx.xxx.x.x.xxx.xx.xx.xx.xx.xxx.xxx.xxx.x.x.xxxx.x.x.x"),
                // Chi Wheels
                Wheel::new(".xx.xx...xx.xx..x....xxx..xxx....xxx..xx."),
                Wheel::new("xx.xx....xxx.xxxx.x...xx..xx..."),
                Wheel::new("..x..xx...xx...xxx...xx.xxxx."),
                Wheel::new("x.x.x..xx...xx..x.xxx..x.x"),
                Wheel::new(".x..xxxx...x.xxx....x.x"),
            ],
        }
    }
}
