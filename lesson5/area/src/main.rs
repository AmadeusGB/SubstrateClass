struct Round {
    pi: f32,
    r:  f32,
}
struct Triangle {
    b: u32,
    h: u32,
}
struct Rectangle {
    w: u32,
    h: u32,
}

trait TypeInfo {
    fn type_of(&self) -> &str;
    fn calc_area(&self);
}

impl TypeInfo for Round {
    fn type_of(&self) -> &str {
        "Round"
    }
    fn calc_area(&self) {
        println!("{:?}",self.pi * self.r * self.r);
    }
}

fn calculate_area<T: TypeInfo + Debug>(task: T) {
    match task.type_of() {
        "Round" => task.calc_area(),
        _ => println!("This type don't implement"),
    }

}

fn main() {
    let example1 = Round { pi:3.14, r:10.0};
    let example2 = Triangle { b:2, h:1};
    let example3 = Rectangle { w:2, h:5};

    //println!("{:?}",example1.type_of());
    calculate_area(example1);


    //println!("Round calculate Result is {:?}", calculate_area(example1));
    //println!("Triangle calculate Result is {:?}", calculate_area(example2.b, example2.h));
    //println!("Rectangle calculate Result is {:?}", calculate_area(example3.w, example3.h));
}
