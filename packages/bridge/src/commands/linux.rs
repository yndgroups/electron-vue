// 求和
#[allow(unused)]
#[napi]
pub fn linux_sum(a: i32, b: i32) -> i32 {
  println!("计算中...");
  a + b
}

// linux 调用隐藏状态栏
#[allow(unused)]
#[napi]
pub fn linux_hide_task_bar(types: u8) -> String {
  match types {
    1 => {
        println!("隐藏任务栏");
        return "隐藏任务栏".to_string();
    }
    2 => {
        println!("显示任务栏");
        return "隐藏任务栏".to_string();
    }
    _ => {
      return "指令错误".to_string();
    }
}
}