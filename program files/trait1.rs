fn main() {
    let car1 = Sedan;
    let car2 = SUV;
    road_tip(&car1);
    road_tip(&car2);
}

struct Sedan;
impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan is driving");
    }
}

struct SUV;
impl LandCapable for SUV {
    fn drive(&self) {
        println!("SUV is driving");
    }
}

trait LandCapable {
    fn drive(&self) {
        println!("Default car is running.");
    }
}

fn road_tip<T: LandCapable>(vehicle: &T) {
    vehicle.drive();
}
