use std::ops::Mul;
struct Round<T> {
    pi: T,
    r:  T,
}
struct Triangle<T> {
    b: T,
    h: T,
}
struct Rectangle<T> {
    w: T,
    h: T,
}


/*
impl<T: Copy + std::ops::Mul<Output = T>> Round<T> {
    fn calc_area(&self) -> T {
        self.pi * self.r * self.r
    }
}*/
/*
impl<T: Mul<Output = T> + Copy> Triangle<T> {
    fn calc_area(&self) -> T {
        self.b * self.h
    }
}
impl<T: Mul<Output = T> + Copy> Rectangle<T> {
    fn calc_area(&self) -> T {
        self.w * self.h
    }
}*/

/*
impl<T: Mul<Output = T> + Copy> HasArea<T> for Triangle<T> {
    fn type_of(&self) -> &str {
        "Triangle"
    }
}
impl<T: Mul<Output = T> + Copy> HasArea<T> for Rectangle<T> {
    fn type_of(&self) -> &str {
        "Rectangle"
    }
}*/
/*
fn calculate_area<T: HasArea>(shape: T) {
    
}*/

fn main() {
    let example1 = Round { pi:3, r:10};
    let example2 = Triangle { b:2.0, h:1.0};
    let example3 = Rectangle { w:2, h:5};

    //calculate_area(example1);
/*
    println!("{:?}",example1.calc_area());
    println!("{:?}",example2.calc_area() * 0.5);
    println!("{:?}",example3.calc_area());*/
}
