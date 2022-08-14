#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

static COUNTER: i32 = 0;

#[tauri::command]
fn get_counter() -> String {
   return COUNTER.to_string();
}

#[tauri::command]
fn inc(mut cnt: i32) -> String {
  cnt+=1;
   return cnt.to_string();
}

#[tauri::command]
fn dec(mut cnt: i32) -> String {
  cnt-=1;
   return cnt.to_string();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![inc, get_counter, dec])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
