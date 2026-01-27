pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|&s| s == 0) {
            return None;
        }
        let [a, b, c] = sides;

        if a + b >= c && b + c >= a && a + c >= b {
            Some(Triangle { a, b, c })
        } else {
            return None;
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.a == self.c || self.b == self.c
    }
}
