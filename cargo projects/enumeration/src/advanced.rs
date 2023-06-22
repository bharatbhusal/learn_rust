#[derive(Debug, Clone, Copy)]
enum Move {
    Still,
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Time(u8, u8, u8);

#[derive(Debug)]
struct MovingMan {
    name: String,
    currection_direction: Move,
    total_time: Time,
    log: Vec<(Move, Time)>,
}

impl MovingMan {
    fn instantiate(name: String, direction: Move, time: Time) -> MovingMan {
        MovingMan {
            name,
            currection_direction: direction,
            total_time: time,
            log: vec![(direction, time)],
        }
    }

    fn update_time(&mut self, time: Time) {
        let mut hours = time.0 + self.total_time.0;
        let mut minutes = time.1 + self.total_time.1;
        let mut seconds = time.2 + self.total_time.2;

        if seconds >= 60 {
            minutes += seconds / 60;
            seconds = seconds % 60;
        }

        if minutes >= 60 {
            hours += minutes / 60;
            minutes = minutes % 60;
        }
        self.total_time.0 = hours;
        self.total_time.1 = minutes;
        self.total_time.2 = seconds;
    }

    fn move_for(&mut self, direction: Move, time: Time) {
        self.log.push((direction, time));
        self.currection_direction = direction.clone();
        self.update_time(time);
    }
}
pub fn advanced() {
    println!("Advanced Enumeration");
    let mut ramesh = MovingMan::instantiate("Ramesh".to_string(), Move::Still, Time(2, 30, 20));
    println!("{:#?}", ramesh);
    ramesh.move_for(Move::Left, Time(1, 59, 40));
    println!("{:#?}", ramesh);
    ramesh.move_for(Move::Up, Time(0, 34, 0));
    println!("{:#?}", ramesh);
    ramesh.move_for(Move::Down, Time(0, 3, 0));
    println!("{:#?}", ramesh);
    ramesh.move_for(Move::Right, Time(1, 0, 0));
    println!("{:#?}", ramesh);
}
