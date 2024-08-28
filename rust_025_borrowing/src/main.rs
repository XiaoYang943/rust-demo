fn main() {
    decryption_of_references();

    immutable_reference();

    variable_reference();

    limitation_one_of_variable_references();

    limitation_two_of_variable_references();

    dangling_references();
}

// 解引用
fn decryption_of_references() {
    let x = 5;  // 变量 x 存放了一个值 5
    let y = &x; // y 是 x 的一个引用

    assert_eq!(5, x);   // 断言 x 等于 5
    assert_eq!(5, *y);  // 如果希望对 y 的值做出断言，必须使用 *y 解引用 来解出引用所指向的值
}

// 不可变引用
fn immutable_reference() {
    let s1 = String::from("hello");
    /*
        用 s1 的引用作为参数传递给 calculate_length 函数，而不是把 s1 的所有权转移给该函数
        & 符号即是引用，允许你使用值，但是不获取所有权
        通过 &s1 语法，我们创建了一个指向 s1 的引用，但是并不拥有它。因为并不拥有这个值，当引用离开作用域后，其指向的值也不会被丢弃
    */
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {  // 函数 calculate_length 使用 & 来表明参数 s 的类型是一个引用
    s.len()
}   // s 离开了作用域。但因为它并不拥有引用值的所有权，所以是安全的

// 可变引用
fn variable_reference() {
    let mut s = String::from("hello");  // 声明 s 是可变类型
    change(&mut s);
}
fn change(some_string: &mut String) {   // 创建一个可变的引用 &mut s 和接受可变引用参数 some_string: &mut String 的函数
    some_string.push_str(", world");
    println!("{}", some_string);
}


/*
   可变引用的限制1：可变引用同时只能存在一个，否则报错：annot borrow `s` as mutable more than once at a time (同一时间无法对 `s` 进行两次可变借用)(编译器 borrow checker 特性)
   该限制的好处：使 Rust 在编译期就避免数据竞争，提高安全性
   */
fn limitation_one_of_variable_references() {
    let mut s = String::from("hello");

    {   // 大括号可以通过手动限制变量的作用域解决编译不通过的问题
        let r1 = &mut s;
        println!("{}", r1);

    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r2 = &mut s;
    println!("{}", r2);
}

/*
   可变引用的限制2：可变引用与不可变引用不能同时存在，否则报错：cannot borrow `s` as mutable because it is also borrowed as immutable (无法借用可变 `s` 因为它已经被借用了不可变)
   */
fn limitation_two_of_variable_references() {
    let mut s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题
    // println!("{}, {}, and {}", r1, r2, r3);


    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
}   // r3作用域在这里结束

/*
    悬垂引用/悬垂指针:
    为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用
    在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：当你获取数据的引用后，编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用。
*/
fn dangling_references() {
    // let res = dangle();
    let res = no_dangle();
    println!("{}", res);
}
/*
    尝试创建一个悬垂引用，Rust 会抛出一个编译时错误
    missing lifetime specifier
    help:this function's return type contains a borrowed value, but there is no value for it to be borrowed from(该函数返回了一个借用的值，但是已经找不到它所借用值的来源)
    因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放，但是此时我们又尝试去返回它的引用。这意味着这个引用会指向一个无效的 String
*/
// fn dangle() -> &String {    // dangle 返回一个字符串的引用
//     let s = String::from("hello");  // s 是一个新字符串
//     &s  // 返回字符串 s 的引用。
// }   // 危险：这里 s 离开作用域并被丢弃。其内存被释放。

fn no_dangle() -> String {
    let s = String::from("hello");

    s   // 解决方法是直接返回 String
}