#[derive(Debug)]
pub enum MessageKind {
    Quit,
    Send { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl MessageKind {
    pub fn writes(&self) -> Option<String> {
        match self {
            MessageKind::Write(content) => Some(format!("Writing {}", content)),
            _ => None,
        }
    }

    pub fn sends(&self) -> Option<String> {
        match self {
            MessageKind::Send { x, y } => Some(format!("Sending to ({}, {})", x, y)),
            _ => None,
        }
    }

    pub fn changes_color(&self) -> Option<String> {
        match self {
            MessageKind::ChangeColor(r, g, b) => {
                Some(format!("Changing color to rgb({}, {}, {})", r, g, b))
            }
            _ => None,
        }
    }
    pub fn quits(&self) -> Option<String> {
        match self {
            MessageKind::Quit => Some("Quiting".to_string()),
            _ => None,
        }
    }
}

pub fn basic() {
    let move_order = MessageKind::Send { x: 54, y: 32 };
    let write_order = MessageKind::Write("Hi mister".to_string());
    let color_change_order = MessageKind::ChangeColor(45, 200, 150);
    let quit_order = MessageKind::Quit;
    println!("Send order: ");
    println!("{:#?}", move_order.sends());
    println!("{:#?}", move_order.writes());
    println!("{:#?}", move_order.changes_color());
    println!("{:#?}", move_order.quits());
    println!("Write order: ");
    println!("{:#?}", write_order.sends());
    println!("{:#?}", write_order.writes());
    println!("{:#?}", write_order.changes_color());
    println!("{:#?}", write_order.quits());
    println!("Color change order: ");
    println!("{:#?}", color_change_order.sends());
    println!("{:#?}", color_change_order.writes());
    println!("{:#?}", color_change_order.changes_color());
    println!("{:#?}", color_change_order.quits());
    println!("Quit order: ");
    println!("{:#?}", quit_order.sends());
    println!("{:#?}", quit_order.writes());
    println!("{:#?}", quit_order.changes_color());
    println!("{:#?}", quit_order.quits());
}
