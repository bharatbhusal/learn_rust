pub struct Shoe {
    pub size: u8,
    pub style: String,
}

pub fn shoe_8(all_shoes: Vec<Shoe>, s: u8) -> Vec<Shoe> {
    let result: Vec<Shoe> = all_shoes
        .into_iter()
        .filter(|shoe| shoe.size == s)
        .collect();

    result
}
