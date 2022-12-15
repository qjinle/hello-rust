fn main() {
    // 可变变量 mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // 常量 必须规定数据类型
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("常量值为: {THREE_HOURS_IN_SECONDS}");

    // 隐藏
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // 元组
    let tup = (500, 1, 6.4, true);

    // let (x, y, z, b) = tup;
    let x = tup.0;

    // 数组
    let arr = [1, 2, 3, 4, 5];
    let arr1 = [3; 5]; // [3, 3, 3, 3, 3]

    println!("The value of x is: {x}");
}