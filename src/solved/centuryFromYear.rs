// fn main() {
//     println!(" CENTURY {}", centuryFromYear(2009))
// }
#[allow(non_snake_case, dead_code)]
pub fn centuryFromYear(year: i32) -> i32 {
    ((year - 1) / 100) + 1
}
