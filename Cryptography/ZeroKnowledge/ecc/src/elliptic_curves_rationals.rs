use num_rational::Rational;
use num_rational::Ratio;

#[derive(Debug)]
pub struct ec
{
    a: Rational,
    b: Rational,
    discriminant: Rational,
}

pub struct point {
    curve: ec,
    x: Rational,
    y: Rational,
}

pub struct ideal {
    curve: ec,
    x: Rational,
    y: Rational,
}

impl ideal {
    pub fn new(curve: ec) -> Self {
        curve: curve,
    }

    pub fn _self_() -> String {
        String::from("Ideal")
    }

    pub fn _neg_(&self) -> Self {
        self
    }

    pub fn _add_(&self, Q: point) -> point {
        Q
    }
}

impl point {
    pub fn new(curve: ec, x: Rational, y: Rational) -> Self {
        if !curve.testPoint(x, y) {
            panic!("The point ({}, {}) is not on the given curve {:?}", x, y, curve);
        }
        point {
            curve: curve,
            x: x,
            y: y,
        }
    }

    // most definitely going to run into memory management problems here
    pub fn _neg_(&self) -> Self {
        point {
            curve: self.curve,
            x: self.x,
            y: -self.y
        }
    }

    pub fn _add_(&self, Q: point) -> Self {
        // PAUSE HERE AND FINISH LATER
        // implement Vieta's formula
        unimplemented!()
    }
}

impl ec  {
    pub fn new(a: Rational, b: Rational) -> Self {
        // check if the curve is smooth based on determinant
        if (Ratio::from_integer(-16) * (( Ratio::from_integer(4) * (a * a * a)) + ( Ratio::from_integer(27) * (b * b)))) == Ratio::from_integer(0) {
            panic!("The curve is not smooth!");
        }
        ec {
            a: a,
            b: a,
            discriminant: (Ratio::from_integer(-16) * (( Ratio::from_integer(4) * (a * a * a)) + ( Ratio::from_integer(27) * (b * b)))),
        }
    }

    pub fn smooth(&self) -> bool {
         // check if the curve is smooth
        if self.discriminant == Ratio::from_integer(0) {
            return false;
        }
        true
    }

    pub fn testPoint(&self, x: Rational, y: Rational) -> bool {
        (y * y) == ( (x * x * x) + (self.a * x) + self.b)
    }

    pub fn _self_(&self) -> String {
        format!("y^2 = x^3 + {}x + {}", self.a, self.b)
    }

    pub fn _eq_(&self, other: ec) -> bool {
        (self.a == other.a) && (self.b == other.b)
    }
}

#[cfg(test)]
mod test {
    use super::ec;
    use num_rational::Ratio;

    #[test]
    fn elliptic_curve() {
        let mut ec_new = ec::new(Ratio::from_integer(17), Ratio::from_integer(1));

        //assert_eq!(true, ec_new.testPoint(Ratio::from_integer(12), Ratio::from_integer(113)));
    }
}

