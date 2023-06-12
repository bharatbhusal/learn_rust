fn main() {
    let duck = Amphibian {};
    let shark = Fish {};
    let pegion = Bird {};
    println!("Duck:");
    duck.swim();
    duck.fly();
    println!("Shark:");
    shark.swim();
    println!("Pegion:");
    pegion.fly();
    magic(&duck);
}
trait Fly {
    fn fly(&self) {
        println!("I can fly.");
    }
}
trait Swim {
    fn swim(&self) {
        println!("I can swim.");
    }
}
// trait Both: Swim + Fly {}

struct Bird {}
struct Amphibian {}
struct Fish {}

impl Fly for Bird {}
impl Swim for Fish {}

// impl Both for Amphibian {}
impl Swim for Amphibian {}
impl Fly for Amphibian {}

fn magic(_animal: &(impl Fly + Swim)) {
    println!("I can do anything.");
}
