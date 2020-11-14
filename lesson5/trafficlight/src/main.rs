#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Light {
    fn time(&self) -> i32;
}

impl Light for TrafficLight{
    fn time(&self) -> i32 {
        return match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    println!("If traffic light is {:?},it will {:?} seconds",red, red.time());
    println!("If traffic light is {:?},it will {:?} seconds",green, green.time());
    println!("If traffic light is {:?},it will {:?} seconds",yellow, yellow.time());
}
