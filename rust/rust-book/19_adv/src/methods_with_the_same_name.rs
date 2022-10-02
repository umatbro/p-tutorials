trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("Flapping hands");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

pub fn main() {
    let person = Human;
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);
}