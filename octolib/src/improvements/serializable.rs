/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804


use helper::printer::print;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32
}

pub fn serialize_test() {

    let point= Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();

}