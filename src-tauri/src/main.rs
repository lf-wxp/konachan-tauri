mod commander;
mod image;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![commander::get_post])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
