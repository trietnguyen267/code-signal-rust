// fn main() {
//     let vec = vec![3, 6, -2, -5, 7, 3];
//     println!("{}", adjacent_elements_product(vec));
// }

fn adjacent_elements_product(input_array: Vec<i32>) -> i32 {
    let mut _result: i32 = std::i32::MIN;
    let mut product: i32 = std::i32::MIN;

    for (i, _item) in input_array.iter().enumerate() {
        if i + 1 < input_array.len() {
            product = input_array[i] * input_array[i + 1];
        }
        if product > _result {
            _result = product;
        }
    }
    return _result;
}

// Best
fn adjacentElementsProduct(inputArray: Vec<i32>) -> i32 {
    inputArray
        .windows(2)
        .map(|pair| pair.iter().product())
        .max()
        .unwrap()
}
