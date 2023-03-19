pub mod hosting; // 通过其他文件引入模块

// 在当前文件引入模块
mod serving {
    fn take_order(){}
    fn serve_order() {}
    pub fn take_money(){}
}

pub fn fix_incrrect_order() {
    cook_order();
    super::server_order(); // 和下面那行等价，可以通过super实现获取父模块的函数
    crate::server_order();
    serving::take_money();
}

pub fn cook_order() {}
