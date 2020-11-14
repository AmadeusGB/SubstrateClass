use std::ops::Mul;
struct Round<T: Copy + Mul> {
    pi: T,
    r:  T,
}
struct Triangle<T: Copy + Mul> {
    b: T,
    h: T,
}
struct Rectangle<T: Copy + Mul> {
    w: T,
    h: T,
}

impl<T: Copy + Mul<Output = T>> Round<T> {
    fn calc_area(&self) -> T {
        self.pi * self.r * self.r
    }
}
impl<T: Mul<Output = T> + Copy> Triangle<T> {
    fn calc_area(&self) -> T {
        self.b * self.h
    }
}
impl<T: Mul<Output = T> + Copy> Rectangle<T> {
    fn calc_area(&self) -> T {
        self.w * self.h
    }
}


fn main() {
    let example1 = Round { pi:3, r:10};
    let example2 = Triangle { b:2.0, h:1.0};
    let example3 = Rectangle { w:2, h:5};

    println!("Round area is {:?}",example1.calc_area());
    println!("Triangle area is {:?}",example2.calc_area() * 0.5);
    println!("Rectangle area is {:?}",example3.calc_area());
}
