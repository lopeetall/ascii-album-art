mod display;
use crate::display::*;

fn main() {
    let path = "atomizer_cover.jpg";
    println!("{}", image_string(path, 24));
}
