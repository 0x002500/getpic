use screenshots::Screen;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn getpic_core(n: i32) -> bool{
    let screens = Screen::all().unwrap();

    for i in 1..n+1 {
        for screen in &screens {
            let mut image = screen.capture().unwrap();
            image.save(format!("{}.png", i)).unwrap();
        }
    }

    return true;
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![getpic_core])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
