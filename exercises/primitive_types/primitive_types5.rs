// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; // 解构元组，将第一个元素赋值给name，第二个元素赋值给age

    println!("{} is {} years old.", name, age);
}
