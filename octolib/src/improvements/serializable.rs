/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804


use helper::printer::print;
extern crate serde;

//#[macro_use]
//extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32
}

pub fn serialize_test() {

//    let point= Point { x: 1, y: 2 };


}