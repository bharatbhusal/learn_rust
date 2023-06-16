pub struct Rectanlge {
    pub leng: u32,
    pub bred: u32,
}

impl Rectanlge {
    pub fn fits(&self, r: &Rectanlge) -> bool {
        (r.leng < self.leng && r.bred < self.bred) || (r.leng < self.bred && r.bred < self.leng)
    }
    pub fn area(&self) -> u32 {
        self.leng * self.bred
    }
    pub fn new(leng: u32, bred: u32) -> Rectanlge {
        Rectanlge { leng, bred }
    }
}
