fn main() {
    numerical_operations();
    println!("");
    bit_operations();
    println!("");
    range();
    println!("");
}
// 序列Range
fn range() {
    for i in 1..=5 {
        println!("{}",i);
    }

    for i in 'a'..='z' {
        println!("{}",i);
    }
}
// 位运算
/**
    运算符	说明
    & 位与	相同位置均为1时则为1，否则为0
    | 位或	相同位置只要有1时则为1，否则为0
    ^ 异或	相同位置不相同则为1，相同则为0
    ! 位非	把位中的0和1相互取反，即0置为1，1置为0
    << 左移	所有位向左移动指定位数，右位补0
    >> 右移	所有位向右移动指定位数，带符号移动（正数补0，负数补1）
**/
fn bit_operations() {
    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}


// 数字运算
fn numerical_operations() {
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}