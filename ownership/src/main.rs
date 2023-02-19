fn main() {
    {
        // 一个block
        let s = "aa,bb,cc";
    }
    let s = "aa,bb,cc";
    println!("s value in block: {s}");

    let mut str_from_std_nspc = String::from("hello");
    println!("string from string name space: {str_from_std_nspc}");
    str_from_std_nspc.push_str(" youn man");
    println!("after push: {str_from_std_nspc}");

    let s1 = String::from("test copy String");
    let s2 = s1;
    //    println!("s1: {s1}"); //会报错，s1已经被释放
    println!("s2: {s2}");

    let mut s3 = s2.clone();
    println!("s2: {s2}"); // 不会报错，s3是s2 clone函数生成的
    println!("s3: {s3}");

    s3 = takes_and_gives_back(s3);
    println!("s3:{s3}");

    take_ownership(s3);
    //    println!("s3:{s3}"); //会报错，因为直接传参导致ownership已经给了形参

    let int_has_copy_trait = 5;
    make_copy(int_has_copy_trait);
    println!("int_has_copy_trait: {int_has_copy_trait}");

    let s4 = s2.clone();
    call_use_reference(&s4);
    println!("s4: {s4}");

    let mut s5 = s4.clone();
    call_use_reference_with_change(&mut s5);

    let s6 = &mut s5;
    let s7 = &mut s5;
    //    println!("s6: {s6}, s7: {s7}"); // 这里会报错，因为变量所有权只能借给一个变量，后借的变量借到最终所有权
    let s8 = &s5;
    //    println!("s7:{s7}"); // 这里会报错，因为可变引用的借用权已经被不可变引用覆盖
    let s9 = &s5;
    println!("s8: {s8}, s9:{s9}"); // 这里没问题，因为不可变引用可以重复获得借用权
    let s10 = &mut s5;
    //    println!("s8: {s8}, s9: {s9}, s10: {s10}"); // 这里会报错，因为可变引用会清除之前所有的借用权
    //    println!("s8: {s8}, s9: {s9}"); // 这里会报错，因为可变引用会清除之前所有的借用权
    println!("s10: {s10}");

    let first_word_end = get_first_word(s10);
    println!("first_word_end: {first_word_end}");

    let s11 = get_sub_str(s10, 0, 5);
    println!("s11: {s11}");
    s10.clear();
    //    println!("s11: {s11}"); // 会报错，因为s10.clear()中获取了s10的可变借用权，导致s11实用slice range获取的不可变借用权失效
    println!("sub slice: {}", get_sub_str_use_slice(&s5[..])) // 使用字符串切片代替字符串引用是更常见的做法
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

fn take_ownership(some_string: String) {
    println!("some_string: {some_string}")
}

fn make_copy(some_integer: i32) {
    println!("some_integer: {some_integer}")
}

fn call_use_reference(ref_s: &String) {
    println!("ref_s: {ref_s}");
    //    ref_s.push_str("try push something"); // 这里会报错，因为引用的只有借用权，不能改变值
}

fn call_use_reference_with_change(ref_s: &mut String) {
    ref_s.push_str("try push something");
    println!("after change: {ref_s}");
}

//fn call_with_dangling_ref() -> &String { // 这种声明会报错，不允许返回悬垂指针
//    let s = String::from("hello");
//
//    &s // 这里返回s的引用
//} // 这里s失效，导致返回的引用不合法，编译失败

fn get_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn get_sub_str(s: &String, start_index: usize, end_index: usize) -> &str {
    &s[start_index..end_index]
}

fn get_sub_str_use_slice(s: &str) -> &str {
    "hello"
}
