fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5    // 提前返回
    }

    x + 5   // 默认表达式返回
}

fn main() {
    let x = plus_or_minus(100);

    println!(" {}", x);
}