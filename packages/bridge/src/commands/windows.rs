use windows::{
  core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
  Win32::UI::WindowsAndMessaging::*,
};

// 调用windows 原生小窗口
fn show_win_dialog(content: String) {
  println!("show_window_native: parmas = {:?}", content);
  let doc: XmlDocument = XmlDocument::new().unwrap();
  doc.LoadXml(h!("<html>hello world</html>")).unwrap();

  let root = doc.DocumentElement().unwrap();
  // assert!(root.NodeName().unwrap() == "html");
  // assert!(root.InnerText().unwrap() == "hello world");
  println!("NodeName {:?}", root.NodeName().unwrap());
  println!("InnerText {:?}", root.InnerText().unwrap());

  unsafe {
    let event = CreateEventW(None, true, false, None).unwrap();
    SetEvent(event).unwrap();
    WaitForSingleObject(event, 0);
    CloseHandle(event).unwrap();
    MessageBoxA(None, s!("native API"), s!("Caption"), MB_OK);
    // MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);
  }
}

// 求和
#[allow(unused)]
#[napi]
pub fn show_window(content: String) {
  println!("显示原生窗口...");
  show_win_dialog(content);
}

// 求和
#[allow(unused)]
#[napi]
pub fn wind_sum(a: i32, b: i32) -> i32 {
  println!("计算中...");
  a + b
}

// windows 调用隐藏状态栏
#[allow(unused)]
#[napi]
pub fn win_hide_task_bar(types: u8) -> String {
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
