// fn main() {
//     assert_eq!(makeArrayConsecutive2(vec![6, 2, 3, 8]), 3);
//     assert_eq!(makeArrayConsecutive2(vec![5, 4, 6]), 0);
//     assert_eq!(makeArrayConsecutive2(vec![0, 3]), 2);
// }

#[allow(non_snake_case, dead_code)]
fn makeArrayConsecutive2(statues: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut array = statues.clone();
    array.sort();
    for (i, item) in array.iter().enumerate() {
        let mut index: i32 = 1;
        while i < array.len() - 1 && item + index < array[i + 1] {
            result = result + 1;
            index = index + 1;
        }
    }
    return result;
}
