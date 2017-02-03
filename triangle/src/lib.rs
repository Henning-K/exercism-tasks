extern crate num;

use num::Num;

#[derive(Debug, PartialEq, Clone)]
pub struct Triangle<T: Copy + PartialOrd + PartialEq + Default + Num> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy + PartialOrd + PartialEq + Default + Num> Triangle<T> {
    pub fn is_equilateral(&self) -> bool {
        if self.x == self.y && self.x == self.z {
            return true;
        }
        false
    }

    pub fn is_isosceles(&self) -> bool {
        if self.is_equilateral() {
            return false;
        }
        if self.x == self.y || self.y == self.z || self.x == self.z {
            return true;
        }
        false
    }

    pub fn is_scalene(&self) -> bool {
        if self.x != self.y && self.y != self.z && self.x != self.z {
            return true;
        }
        false
    }

    pub fn build(inp: [T; 3]) -> Result<Self, ()> {
        let temp = Triangle::new(&inp);

        if inp.iter().all(|i| *i > *i - *i) && temp.z <= temp.x + temp.y &&
           temp.x <= temp.z + temp.y && temp.y <= temp.x + temp.z {
            Ok(temp)
        } else {
            Err(())
        }
    }

    fn new(inp: &[T; 3]) -> Self {
        let inp = inp.clone();
        Triangle {
            x: inp[0],
            y: inp[1],
            z: inp[2],
        }
    }
}
