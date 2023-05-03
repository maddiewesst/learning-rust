#[derive(Debug, Clone)]
pub struct Person<'a>{
	pub name: &'a str,
	pub age: u8,
}

impl<'a> Person<'a> {
	pub fn new(name: &'a str) -> Person {
        Person {
            name,
            age: 0,
        }
	}
}


// #[derive(Debug)]
// pub struct Person{
//     pub name: &'static str,
//     pub age: u8,
// }

// impl Person {
//     pub fn new(name: &'static str) -> Person {
//         Person { name, age: 0 }
//     }
// }