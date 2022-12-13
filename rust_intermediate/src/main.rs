pub mod figures;
pub mod animals;

use crate::figures::*;
use crate::animals::*;

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

    // Enumerations
    let rect = Figure::Rectangle;
    let tria = Figure::Triangle;
    println!("{:?}", rect);
    println!("{:?}", tria);

    // Enumerations with structs
    let cat = Animal::Cat(CatProperty{ meow: String::from("Meow!")});
    let dog = Animal::Dog(DogProperty{ bark: String::from("Bark!")});
    let mut animals : Vec<Animal> = Vec::new();
    animals.push(cat);
    animals.push(dog);

    for animal in &animals {
        match animal {
            Animal::Cat(cat) => {
                println!("Cat found: {}", cat.meow);
            }
            Animal::Dog(dog) => {
                println!("Dog found: {}", dog.bark);
            }
        }
    }

    // Collections - vector
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(50);

    println!("{}", &v[0]);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Third value: {}", third),
        None => println!("Third value missing"),
    }

    v.remove(0);

    for item in &mut v {
        *item /= 10;
    }

    for item in &v {
        println!("{}", item);
    }
}
