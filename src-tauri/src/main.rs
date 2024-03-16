// 防止发布时 Windows 上出现额外的控制台窗口，请勿删除！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, Window};

// MacOS的窗口效果 
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
#[allow(unused)] // window窗口效果
use window_vibrancy::apply_blur;

pub mod menu;

// 了解有关 Tauri 命令的更多信息，请访问 https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!!", name)
}

#[tauri::command]
fn reverse_text(text: &str) -> String {
    text.chars().rev().collect()
}

#[tauri::command]
async fn close_splashscreen(window: Window) {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
    // 默认启动
    tauri::Builder::default()
        // 自定义菜单
        .menu(menu::MenuBuilder::new())
        // 菜单事件
        .on_menu_event(menu::MenuBuilder::on_click)
        // 调用接口
        .invoke_handler(tauri::generate_handler![
            greet,
            reverse_text,
            close_splashscreen
        ])
        // 启动执行
        .setup(|app: &mut tauri::App| {
            // 窗口效果：https://github.com/tauri-apps/window-vibrancy
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(8.0))
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            // （开屏界面）
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                println!("Initializing...");
                std::thread::sleep(std::time::Duration::from_secs(2));
                println!("Done initializing.");
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
