use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x:i32,
    y:i32
}

#[test]
fn serialize(){
    let expected = r#"{"x":1,"y":2}"#;

    let point = Point { x: 1, y: 2 };
    let json = serde_json::to_string(&point).unwrap();
    assert_eq!(expected, json);

    let expected_point = serde_json::from_str::<Point>(expected).unwrap();
    assert_eq!(expected_point.x, point.x);
    assert_eq!(expected_point.y, point.y);
}