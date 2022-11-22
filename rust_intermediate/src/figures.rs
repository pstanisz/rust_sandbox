#[derive(Debug)]
pub struct Rectangle {
    pub width: u64,
    pub height : u64,
}

#[derive(Debug)]
pub struct Triangle {
    pub base: f64,
    pub height : f64,
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

impl Triangle {
    pub fn area(&self) -> f64 {
        0.5 * self.base * f64::from(self.height)
    }
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

#[derive(Debug)]
pub enum Figure {
    Rectangle,
    Triangle
}
