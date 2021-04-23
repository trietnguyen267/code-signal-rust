mod solved;

fn main() {
    assert_eq!(almostIncreasingSequence(vec![1, 3, 2, 1]), false);
    assert_eq!(almostIncreasingSequence(vec![5, 4, 6]), true);
    assert_eq!(almostIncreasingSequence(vec![1, 3, 2]), true);
}

#[allow(non_snake_case)]
fn almostIncreasingSequence(sequence: Vec<i32>) -> bool {
    let mut fail: i16 = 0;
    let mut check = std::i32::MIN;
    for (_i, item) in sequence.iter().enumerate() {
        if item > &check {
            check = *item;
        } else {
            fail = fail + 1;
            check = sequence[_i - 1];
        }

        if fail == 2 {
            return false;
        }
    }
    true
}
