
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

/// [std::ops::Add - Rust](https://doc.rust-lang.org/std/ops/trait.Add.html)
impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

///[std::ops::Sub - Rust](https://doc.rust-lang.org/std/ops/trait.Sub.html)
impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output{
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// [std::ops::Div - Rust](https://doc.rust-lang.org/std/ops/trait.Div.html)
impl std::ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output{
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

/// [std::ops::Mul - Rust](https://doc.rust-lang.org/std/ops/trait.Mul.html)
impl std::ops::Mul for Vector {
    type Output = f32;

    fn mul(self, other: Vector) -> Self::Output {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl Vector{
    /// Vector Length (L2 Norm)
    pub fn norm(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let l = (x*x + y*y).sqrt();
        l
    }

    /// normalized vector
    pub fn normalized(&self) -> Vector {
        Vector {
            x: self.x / self.norm(),
            y: self.y / self.norm(),
        }
    }
}

#[cfg(test)]
mod test_vector {
    
    use super::Vector;

    #[test]
    fn test_create(){
        let vec = Vector{x:0.0, y:1.0};
        assert_eq!(vec.x, 0.0);
        assert_eq!(vec.y, 1.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vector{x:0.0, y:1.0};
        let v2 = Vector{x:1.0, y:0.0};
        let v3 = v1 + v2;
        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, 1.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector{x:0.0, y:1.0};
        let v2 = Vector{x:1.0, y:0.0};
        let v3 = v1 - v2;
        assert_eq!(v3.x, -1.0);
        assert_eq!(v3.y, 1.0);
    }

    #[test]
    fn test_div(){
        let v1 = Vector{x:0.0, y:2.0};
        let v2 = v1 / 2.0;
        assert_eq!(v2.x, 0.0);
        assert_eq!(v2.y, 1.0);
    }
    #[test]
    fn test_divide_zero() {
        // Not Implemented
    }

    #[test]
    fn test_mul(){
        let v1 = Vector{x:1.0, y:1.0};
        let v2 = Vector{x:1.0, y:1.0};

        let m = v1 * v2;

        assert_eq!(m, 2.0);

    }

    #[test]
    fn test_eq() {
        assert_eq!( Vector{x:0.0, y:1.0} , Vector{x:0.0, y:1.0});
        assert_ne!( Vector{x:0.0, y:1.0} , Vector{x:1.0, y:1.0});
    }

    #[test]
    fn test_norm(){
        let v1 = Vector{x:0.0, y:1.0};
        assert_eq!(v1.norm(), 1.0);

        let v2 = Vector{x:1.0, y:1.0};
        assert_eq!(v2.norm(), (2.0f32).sqrt());
    }

    #[test]
    fn test_normalized(){
        let v1 = Vector{x:1.0, y:1.0};
        let v2 = v1.normalized();
        assert_eq!(v2.x, 1.0/ (2.0f32).sqrt());
        assert_eq!(v2.y, 1.0/ (2.0f32).sqrt());
    }

}