fn main() {
    let num = 15;
    println!("test if result, num: {}, res: {}", num, test_if(num));

    let cond = true;
    println!(
        "test condition, con: {}, res: {}",
        cond,
        test_condition(cond)
    );

    let loop_num = 10;
    println!("test loop, num: {}, res: {}", loop_num, test_loop(loop_num));

    let loop_num = 10;
    println!("test loop label, num: {}, res: {}", loop_num, test_loop_label(loop_num));

    let while_loop_num = 10;
    test_while(while_loop_num);
    println!("test while loop, num: {}", while_loop_num);

    test_for(&[1,3,4,5,6,7])

}

fn test_if(num: i32) -> i32 {
    if num > 5 {
        num
    } else if num == 0 {
        num + 1
    } else {
        num * 2
    }
}

fn test_condition(con: bool) -> i32 {
    let num = if con { 5 } else { 6 };
    num
}

fn test_loop(loop_num: i32) -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == loop_num {
            break counter * 2;
        }
    }
}

fn test_loop_label(loop_num: i32) -> i32 {
    let mut count = 0;

    let mut res = 0;
    'counting_up: loop {
        println!("count= {count}");
        let mut loop_count = 0;
        loop {
            if count == 5 {
                break 'counting_up;
            }

            if loop_count == loop_num {
                break;
            }
            res += 1;
            loop_count += 1;
        }
        count += 1
    }
    return res;
}

fn test_while(test_num: i32) {
    let mut loop_num = test_num;
    while loop_num != 0 {
        loop_num -= 1
    }
    println!("break while")
}

fn test_for(nums: &[i32]) {
    for number in nums {
        println!("got num in nums: {number}")
    }

    for rev_num in nums.iter() {
        println!("rev num: {rev_num}")
    }

    for number in (1..4).rev() {
        println!("tuple num: {number}!");
    }
}
