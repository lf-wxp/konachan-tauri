use tauri::Manager;

mod commander;
mod image;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commander::get_post,
      commander::download_image,
      commander::close_splashscreen,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
