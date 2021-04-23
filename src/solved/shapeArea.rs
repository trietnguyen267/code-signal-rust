// fn main() {
//     assert_eq!(shapeArea(1), 1);
//     assert_eq!(shapeArea(2), 5);
//     assert_eq!(shapeArea(3), 13);
// }

#[allow(non_snake_case)]
fn shapeArea(n: i32) -> i32 {
    return n.pow(2) + (n - 1).pow(2);
}
