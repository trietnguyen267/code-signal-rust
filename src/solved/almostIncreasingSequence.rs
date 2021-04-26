#[allow(non_snake_case, dead_code)]
fn almostIncreasingSequence(sequence: Vec<i32>) -> bool {
    let max = *sequence.iter().max().unwrap();
    for _i in 0..sequence.len() - 1 {
        if sequence[_i + 1] <= sequence[_i] {
            if _i + 1 == sequence.len() - 1 {
                return true;
            };
            let mut vec = sequence.clone();
            if sequence[_i] == max {
                vec.remove(_i);
            } else {
                vec.remove(_i + 1);
            }
            for index in 0..vec.len() - 1 {
                if vec[index + 1] <= vec[index] {
                    return false;
                }
            }
        }
    }
    true
}
