fn main() {
    // 可变变量
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);

    const CC: i32 = 1_1;
    println!("{}", CC);

    // 隐藏 shadowing
    let y = 5;
    let y = y + 1;

    {
        let mut y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
        y = 1233;
        println!("The value of mut y in the inner scope is: {}", y);
    }

    println!("The value of x is: {}", y);

    // shadowing 的变量，类型是可变的
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // string类型转换
    let num_42: u8 = match "3".parse() {
        Ok(num) => num,
        Err(err) => {
            println!("err: {}", err);
            return;
        }
    };
    println!("num: {}", num_42);

    let num_f = 0xf;
    println!("num_f: {}", num_f);

    let f_01: f32 = 0.1;
    let f_02: f32 = 0.2;
    println!("sum: {}", f_01 + f_02);

    let double_03: f64 = 0.22;
    println!("double: {}", double_03);

    let n: f32 = 2.0/3.0;
    println!("n: {}", n);

    let ch: char = '😊';
    println!("ch: {}", ch);


    let tup = (1,0.1, "222", 'a');
    let (x, y ,z, k) = tup;
    println!("tup, {} {} {} {}", tup.0, tup.1, tup.2, tup.3);
    println!("tup, {} {} {} {}", x,y,z,k);

    let _uni_type = ();
//    println!("unit type: {}", uni_type); ERROR

    let arr: [i32;5]=[1, 2, 3, 4, 5];
    println!("arr[0] {}", arr[0]);

    let arr = [3;5];
    println!("arr[0] {}", arr[0]);

    // 测试溢出，如果溢出是显式的，那编译检查时，就直接不通过
    // let overflow: u8 = 255;
    // 如果溢出是隐式的，debug运行时，会panic；release后，运行二进制，程序不会报错，运行会进入循环模式
    let overflow: u8 = "255".parse().expect("msg");
    let overflow = overflow + 1;
    println!("overflow: {}", overflow);
}
