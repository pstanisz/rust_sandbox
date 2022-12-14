# Rust intermediate

This is a collection of code snippets presenting some Rust features, intermediate level

## Structs

### Simple structure

Note: Nice feature to enable printing debug information: `#[derive(Debug)]` and `{:?}`

```rust
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u64,
        pub height : u64,
    }

    pub fn area(rect: &Rectangle) -> u64 {
        rect.width * rect.height
    }

    let rect = Rectangle {
        width: 1, 
        height: 2
    };
    let area = area(&rect);
    println!("{:?} of area {}", rect, area);
```

### Field init shorthand

No need to use: `width: width, height: height`

```rust
    // Field init shorthand
    pub fn create_rect(width: u64, height: u64) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }

    let rect2 = create_rect(10, 5);
    println!("{:?}", rect2);
```

### Struct update syntax

No need to use: `height: rect2.height`

```rust
    let rect3 = Rectangle {
        width: 100,
        ..rect2
    };
    println!("{:?}", rect3);
```

### Tuple struct

Structures with unnamed fields, looking as a tuples

```rust
    #[derive(Debug)]
    pub struct RGB(pub u8, pub u8, pub u8);

    let color = RGB(255, 255, 0);
    println!("{:?}", color);    //RGB(255, 255, 0)
```

### Unit-like structs without fields
    
```rust
    #[derive(Debug)]
    pub struct AlwaysRed;

    let red = AlwaysRed;
    println!("{:?}", red);  // AlwaysRed
```

### Method syntax

```rust
    pub struct Rectangle {
        pub width: u64,
        pub height : u64,
    }

    impl Rectangle {
        pub fn area(&self) -> u64 {
            self.width * self.height
        }
    }

    let area2 = rect2.area();
    let area3 = rect3.area();
    println!("rect2 area: {}, rect3 area: {}", area2, area3);   // rect2 area: 50, rect3 area: 500
```

### Associated functions

```rust
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

    let small = Rectangle::small_rect();
    let big = Rectangle::big_rect();
    println!("Small: {:?}, big: {:?}", small, big); // Small: Rectangle { width: 1, height: 1 }, big: Rectangle { width: 100, height: 100 }
```

## Enumerations

### Simple enumeration

```rust
    #[derive(Debug)]
    pub enum Figure {
        Rectangle(Rectangle),
        Triangle(Triangle)
    }

    let rect = Figure::Rectangle;
    let tria = Figure::Triangle;
    println!("{:?}", rect); // Rectangle
    println!("{:?}", tria); // Triangle
```

### Enumeration with structure representation

```rust
    #[derive(Debug)]
    pub struct CatProperty {
        pub meow: String
    }

    #[derive(Debug)]
    pub struct DogProperty {
        pub bark: String
    }

    #[derive(Debug)]
    pub enum Animal {
        Cat(CatProperty),
        Dog(DogProperty)
    }

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

    // Cat found: Meow!
    // Dog found: Bark!
```

## Collections

### Vector

```rust
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(50);

    println!("{}", &v[0]);  // 10
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Third value: {}", third),
        None => println!("Third value missing"),    // here
    }

    v.remove(0);

    for item in &mut v {
        *item /= 10;
    }

    for item in &v {
        println!("{}", item);   // 5
    }
```