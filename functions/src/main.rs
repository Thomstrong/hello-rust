fn main() {
    println!("Hello, world!");
    first_function();
    func_with_param(123, "gogogo");
    test_express_var();
    println!("function return: {}", func_with_return())
}

fn first_function() {
    println!("this is first function")
}

fn func_with_param(int_param: i32, str_param: &str) {
    println!("int_param: {}", int_param);
    println!("str_param: {}", str_param);
}

fn test_express_var() {
    let x = 5;
    let y  = {
        let x = 3;
        x + 1 // 这里不能加分号
    };

    println!("expression_y: {}, x: {}", y, x)
}

fn func_with_return() -> i32 {
    let x = 13;
    x + 23
}
