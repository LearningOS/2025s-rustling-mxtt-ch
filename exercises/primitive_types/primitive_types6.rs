// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // 使用元组索引语法替换下面的 ???
    let second = numbers.1; // 在Rust中，元组索引从0开始，所以第二个元素的索引是1

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
