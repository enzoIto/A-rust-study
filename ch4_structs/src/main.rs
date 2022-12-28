fn main() {
    let t = Triangle::new(3.0,4.0,5.0);
    let a = t.area();
    let t_type = t.triangle_type;

}

enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene
}

struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
    triangle_type: TriangleType
}


impl Triangle {
    fn new(side1: f64, side2: f64, side3: f64) -> Triangle {
        let triangle_type = if side1 == side2 && side2 == side3 {
            TriangleType::Equilateral
        } else if side1 == side2 || side2 == side3 || side1 == side3 {
            TriangleType::Isosceles
        } else {
            TriangleType::Scalene
        };

        Triangle {side1,side2,side3,triangle_type}
    }
    fn area(&self) -> f64 {
        let s = (self.side1 + self.side2 + self.side3) / 2.0;
        (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt()
    }
}
