pub struct Guess {
    value: u8,
}

impl Guess {
    pub fn new(value: u8) -> Guess {
        if value < 10 {
            panic!("Guess is lesser than valid guess");
        } else if value > 100 {
            panic!("Guess is greater than valid guess");
        }
        Guess { value }
    }
}
