use super::super::OCTAVE_MULTIPLICATIVE;
use std::fmt;

pub const OCTAVE_UP: Proportion = Proportion {
    magnitude_a: 1,
    magnitude_b: OCTAVE_MULTIPLICATIVE as u32,
    magnitude_a_norm: 1,
    magnitude_b_norm: OCTAVE_MULTIPLICATIVE as u32,
};

pub const OCTAVE_DOWN: Proportion = Proportion {
    magnitude_a: OCTAVE_MULTIPLICATIVE as u32,
    magnitude_b: 1,
    magnitude_a_norm: OCTAVE_MULTIPLICATIVE as u32,
    magnitude_b_norm: 1,
};

pub const UNIT: Proportion = Proportion {
    magnitude_a: 1,
    magnitude_b: 1,
    magnitude_a_norm: 1,
    magnitude_b_norm: 1,
};

#[derive(Eq, Debug)]
pub struct Proportion {
    magnitude_a: u32,
    magnitude_b: u32,
    magnitude_a_norm: u32,
    magnitude_b_norm: u32,
}

impl Proportion {
    pub fn new(magnitude_a: u32, magnitude_b: u32) -> Proportion {
        let mut prop = Proportion {
            magnitude_a,
            magnitude_b,
            magnitude_a_norm: magnitude_a,
            magnitude_b_norm: magnitude_b,
        };

        prop.normalize();

        return prop;
    }

    fn normalize(&mut self) {
        let mut r;
        let mut a = self.magnitude_a;
        let mut b = self.magnitude_b;

        while (a % b) > 0 {
            r = a % b;
            a = b;
            b = r;
        }

        self.magnitude_a_norm = self.magnitude_a / b;
        self.magnitude_b_norm = self.magnitude_b / b;
    }

    pub fn fusion(&self, other: &Proportion) -> Proportion {
        Proportion::new(
            self.magnitude_a * other.magnitude_a,
            self.magnitude_b * other.magnitude_b,
        )
    }

    pub fn invert(&self) -> Proportion {
        Proportion::new(self.magnitude_b, self.magnitude_a)
    }

    pub fn pow(&self, power: i32) -> Proportion {
        if power == 0 {
            UNIT
        } else {
            if power < 0 {
                Proportion::new(
                    self.magnitude_b.pow(power.abs() as u32),
                    self.magnitude_a.pow(power.abs() as u32),
                )
            } else
            /* power > 0 */
            {
                Proportion::new(
                    self.magnitude_a.pow(power.abs() as u32),
                    self.magnitude_b.pow(power.abs() as u32),
                )
            }
        }
    }

    pub fn scale(&self, number: f64) -> f64 {
        (number * self.magnitude_b as f64) / self.magnitude_a as f64
    }
}

impl PartialEq<Proportion> for Proportion {
    fn eq(&self, rhs: &Proportion) -> bool {
        self.magnitude_a_norm == rhs.magnitude_a_norm
            && self.magnitude_b_norm == rhs.magnitude_b_norm
    }
}

impl fmt::Display for Proportion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.magnitude_a, self.magnitude_b)
    }
}

#[cfg(test)]
mod tests {
    use super::Proportion;

    #[test]
    fn display_test() {
        assert_eq!(format!("{}", Proportion::new(1, 2)), "1:2");
        assert_eq!(format!("{}", Proportion::new(2, 1)), "2:1");
    }

    #[test]
    fn fusion_test() {
        let a = Proportion::new(2, 3);
        let b = Proportion::new(3, 4);
        let c = a.fusion(&b);
        assert_eq!(format!("{}", c), "6:12");
    }

    #[test]
    fn equivalence_test() {
        let a = Proportion::new(1, 2);
        let b = Proportion::new(2, 4);
        assert_eq!(a, a, "a = {}", a);
        assert_eq!(a, b, "a = {}, b = {}", a, b);

        let b = Proportion::new(4, 2);
        assert_ne!(a, b, "a = {}, b = {}", a, b);

        let a = Proportion::new(5, 3);
        let b = Proportion::new(15, 9);
        assert_eq!(a, a, "a = {}", a);
        assert_eq!(a, b, "a = {}, b = {}", a, b);

        let b = Proportion::new(14, 9);
        assert_ne!(a, b, "a = {}, b = {}", a, b);
    }

    #[test]
    fn invert_test() {
        let a = Proportion::new(2, 3);
        let b = Proportion::new(3, 4);
        assert_eq!(format!("{}", a.invert()), "3:2");
        assert_eq!(format!("{}", b.invert()), "4:3");
    }

    #[test]
    fn pow_test() {
        let a = Proportion::new(2, 3);
        assert_eq!(format!("{}", a.pow(0)), "1:1");
        assert_eq!(format!("{}", a.pow(-1)), "3:2");
        assert_eq!(format!("{}", a.pow(-3)), "27:8");
        assert_eq!(format!("{}", a.pow(1)), "2:3");
        assert_eq!(format!("{}", a.pow(3)), "8:27");
    }

    #[test]
    fn scale_up_test() {
        let a = Proportion::new(2, 3);
        let b = Proportion::new(4, 6);
        assert_eq!(format!("{:.3?}", a.scale(6.000)), "9.000");
        assert_eq!(format!("{:.3?}", b.scale(6.000)), "9.000");
        assert_eq!(format!("{:.3?}", a.scale(3.000)), "4.500");
        assert_eq!(format!("{:.3?}", b.scale(3.000)), "4.500");
        assert_eq!(format!("{:.3?}", a.scale(3.333)), "5.000");
        assert_eq!(format!("{:.3?}", b.scale(3.333)), "5.000");
        assert_eq!(format!("{:.3?}", a.scale(3.251)), "4.877");
        assert_eq!(format!("{:.3?}", b.scale(3.251)), "4.877");
    }

    #[test]
    fn scale_down_test() {
        let a = Proportion::new(3, 2);
        let b = Proportion::new(6, 4);
        assert_eq!(format!("{:.3?}", a.scale(6.000)), "4.000");
        assert_eq!(format!("{:.3?}", b.scale(6.000)), "4.000");
        assert_eq!(format!("{:.3?}", a.scale(2.000)), "1.333");
        assert_eq!(format!("{:.3?}", b.scale(2.000)), "1.333");
        assert_eq!(format!("{:.3?}", a.scale(3.333)), "2.222");
        assert_eq!(format!("{:.3?}", b.scale(3.333)), "2.222");
    }
}
