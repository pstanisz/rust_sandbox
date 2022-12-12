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
