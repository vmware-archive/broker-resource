extern crate rustc_serialize;

use rustc_serialize::json;

fn main() {
    let empty: Vec<()> = vec![];
    let encoded: String = json::encode(&empty).unwrap();

    print!("{}", encoded);
}
