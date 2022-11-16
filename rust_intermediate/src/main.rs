pub mod figures;

use crate::figures::*;

fn main() {
    let rect = Rectangle {
        width: 1, 
        height: 2
    };
    let area = area(&rect);
    println!("{:?} of area {}", rect, area);

    let rect2 = create_rect(10, 5);
    println!("{:?}", rect2);

    let rect3 = Rectangle {
        width: 100,
        ..rect2
    };
    println!("{:?}", rect3);

    // Using method syntax
    let area2 = rect2.area();
    let area3 = rect3.area();
    println!("rect2 area: {}, rect3 area: {}", area2, area3);

    // Associated functions
    let small = Rectangle::small_rect();
    let big = Rectangle::big_rect();
    println!("Small: {:?}, big: {:?}", small, big);
    
    // Tuple structs with unnamed fields
    let color = RGB(255, 255, 0);
    println!("{:?}", color);

    // Unit-like structs without fields
    let red = AlwaysRed;
    println!("{:?}", red);
}
