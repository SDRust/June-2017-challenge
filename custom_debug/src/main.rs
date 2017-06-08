
#[macro_use]
extern crate custom_debug;

#[derive(CustomDebug)]
struct SomeStructure {
    number: u32,
    string: String,
    float: f32,
}

fn main() {
    println!("{:?}",
             SomeStructure {
                 number: 5,
                 string: "Hello world!".to_owned(),
                 float: 3.14159265358979,
             });
}
