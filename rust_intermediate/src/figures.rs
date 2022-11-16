#[derive(Debug)]
pub struct Rectangle {
    pub width: u64,
    pub height : u64,
}

// Method syntax
impl Rectangle {
    pub fn area(&self) -> u64 {
        self.width * self.height
    }

    // Associated functions
    pub fn small_rect() -> Self {
        Self {
            width: 1,
            height: 1,
        }
    }

    pub fn big_rect() -> Self {
        Self {
            width: 100,
            height: 100,
        }
    }
}

pub fn area(rect: &Rectangle) -> u64 {
    rect.width * rect.height
}

// Field init shorthand
pub fn create_rect(width: u64, height: u64) -> Rectangle {
    Rectangle {
        width,
        height
    }
}

#[derive(Debug)]
pub struct RGB(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct AlwaysRed;
