fn main() {
    let result = add_with_extra(1,1);
    println!("{}",result);

    add();

    assert_eq!(ret_unit_type(), ());
}

// Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 1; // 语句
    x + y // 表达式 不能加分号
}

fn add() {
    let y = {   // 因为 X + 1 是表达式，所以该语句块也是表达式
        let x = 3; // 语句
        x + 1   // 表达式 不能加分号
    };

    println!("{}", y);
}

fn ret_unit_type() {
    let x = 2;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    let z = if x % 2 == 1 { "奇数" } else { "偶数" };
    println!("{}", z);
}
