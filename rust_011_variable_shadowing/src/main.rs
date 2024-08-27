fn main() {
    let x = 5;

    /**
        在main函数的作用域内对之前的x进行遮蔽
        这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配
        而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好
    */
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("x的值: {}", x); // 12
    }

    println!("x的值: {}", x); // 6
}
