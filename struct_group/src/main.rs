use crate::shaps::rectangle::Rectangle;
pub mod shaps;

fn main() {
    let mut user = User {
        active: true,
        username: String::from("test_user"),
        email: String::from("test@test.com"),
        sign_in_count: 0,
    };
    user.email = "test_change@test.com".to_string();
    println!("user: name: {}, email: {}", user.username, user.email);

    let user_with_simple_code = build_user(user.email.clone(), user.username.clone());
    println!(
        "user_with_simple_code: name: {}, email: {}",
        user_with_simple_code.username, user_with_simple_code.email
    );

    let user_build_from_other = build_from_user(user);
    println!(
        "user_build_from_other: name: {}, email: {}",
        user_build_from_other.username, user_build_from_other.email
    );
    //    println!("user: name: {}, email: {}", user.username, user.email); // 这里会报错，因为user的所有权已经在上面函数中转移到临时变量中了

    let mut user_get_some_field_from_other = User {
        username: String::from("new_build_user@test.com"),
        ..user_build_from_other
    };
    println!(
        "user_get_some_field_from_other: name: {}, email: {}",
        user_get_some_field_from_other.username, user_get_some_field_from_other.email
    );
    // 这里因为user_build_from_other的active 实现了copy trait 所以所有权没有转交、email和username没有用到，所有权也没有转交，所以user整体仍然可用
    println!(
        "user_build_from_other: name: {}, active: {}",
        user_build_from_other.username, user_build_from_other.active
    );
    // println!(
    //     "user_build_from_other: email: {}",
    //     user_build_from_other.email
    // ); // 这里会报错，因为email在build新的user的时候，所有权已经转交了

    let black = Color(0, 0, 0);
    println!("R: {} G: {} B: {}", black.0, black.1, black.2);

    println!(
        "user_get_some_field_from_other: {:?}",
        user_get_some_field_from_other
    );
    dbg!(&user_get_some_field_from_other); // dbg 宏会获取参数所有权

    println!(
        "get username of user_get_some_field_from_other: {}",
        user_get_some_field_from_other.get_user_name()
    );
    user_get_some_field_from_other.attach_name(".cn");
    println!(
        "get username of user_get_some_field_from_other: {}",
        user_get_some_field_from_other.get_user_name()
    );

    let admin = User::admin("caicai");
    println!("admin: name: {}, email: {}", admin.username, admin.email);

    let rec = Rectangle{
        width: 3,
        length: 4
    };

    println!("rectangle: {:#?}, area: {}", rec, rec.getArea())
}

#[derive(Debug)] // 使用外部属性标记，使得println!宏可以识别到{:?}占位符
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_from_user(user: User) -> User {
    User {
        email: String::from("new_user@test.com"),
        ..user // 必须放在最后
    }
}

struct Color(i32, i32, i32);

struct UnitStruct;

impl User {
    fn get_user_name(&self) -> &str {
        &self.username[..]
    }

    fn attach_name(self: &mut Self, subfix: &str) {
        // &mut self是self: &mut Self这种方式的缩写
        self.username.push_str(subfix)
    }

}

// 一个struct可以有多个impl块
impl User {
    fn admin(username: &str) -> User {
        let mut email = String::from(username);
        email.push_str("@admin.com");
        User {
            username: String::from(username),
            email,
            active: true,
            sign_in_count: 0,
        }
    }
}

impl User {
    //    fn get_user_name(&self) -> &str { // 这里会报错，允许使用多个impl代码块，但是实现的方法需要是唯一的
    //        &self.email[..]
    //    }
}
