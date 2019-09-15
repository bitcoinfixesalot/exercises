pub struct Triangle {
    side_a: u64,
    side_b: u64,
    side_c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sides = sides;
        sides.sort();

        if sides[0] < 1 || (sides[0] + sides[1]) < sides[2] {
            None
        } else {
            // Triangle with sorted sides
            Some(Triangle {
                side_a: sides[0],
                side_b: sides[1],
                side_c: sides[2],
            })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.side_a == self.side_c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.side_a == self.side_b || self.side_b == self.side_c
    }
}
