#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub length: u32,
}

impl Rectangle {
    pub fn getArea(&self) -> u32 {
        self.width * self.length
    }
}

