// fn main() {
//     println!(" CENTURY {}", centuryFromYear(2009))
// }

fn centuryFromYear(year: i32) -> i32 {
    ((year - 1) / 100) + 1
}
