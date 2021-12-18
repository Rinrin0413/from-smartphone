#![allow(unused)]
fn main() {
    struct Person {
        name: String,
        age: u32,
        gender: String,
        birthday: Birthday,
        origin_country: String,
    }
    struct Birthday { year: u32, month: u8, date: u8, }
    
    let mut rinrin = Person {
        name: String::from("Rinrin"),
        age: 15,
        gender: String::from("male"),
        birthday: Birthday {
            year: 2006, 
            month: 4, 
            date: 13, 
        },
        origin_country: String::from("Japan"),
    };
}