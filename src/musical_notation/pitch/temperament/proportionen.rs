use std::fmt;

#[derive(Eq)]
pub struct Proportion {
    magnitude_a: u16,
    magnitude_b: u16,
    magnitude_a_norm: u16,
    magnitude_b_norm: u16,
}

impl Proportion {
    pub fn new(magnitude_a: u16, magnitude_b: u16) -> Proportion {
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
    fn equequivalence_test() {
        let a = Proportion::new(1, 2);
        let b = Proportion::new(2, 4);
        assert!(a == a, "a = {}", a);
        assert!(a == b, "a = {}, b = {}", a, b);

        let b = Proportion::new(4, 2);
        assert!(a != b, "a = {}, b = {}", a, b);

        let a = Proportion::new(5, 3);
        let b = Proportion::new(15, 9);
        assert!(a == a, "a = {}", a);
        assert!(a == b, "a = {}, b = {}", a, b);

        let b = Proportion::new(14, 9);
        assert!(a != b, "a = {}, b = {}", a, b);
    }
}
