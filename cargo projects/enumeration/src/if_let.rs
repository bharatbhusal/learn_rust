pub enum Dice {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

//prints nothing if val is None.
// prints "Value is {val}" if val is Some(val).
pub fn if_let(val: Option<u8>) {
    if let Some(inp) = val {
        println!("Value is {}", inp);
    }
}

//if let in custom enum.
pub fn which_face(face: Dice) {
    if let Dice::Two = face {
        println!("The face is two");
    }
}

// Which is equivalent to:
// pub fn which_face(face: Dice) {
//     match face {
//         Dice::Two => println!("The face is two"),
//         _ => (),
//     }
// }

//prints "Value is None" if val is other variant than Some(T). In this case None.
// prints "Value is {val}" if val is Some(val).
pub fn this_or_that(val: Option<u8>) {
    if let Some(inp) = val {
        println!("Value is {}", inp);
    } else {
        println!("Value is None");
    }
}

//which is equivalent to:
// pub fn this_or_that(val: Option<u8>) {
//     match val {
//         Some(inp) => println!("Value is {}", inp),
//         None => println!("Value is None"),
//     }
// }
