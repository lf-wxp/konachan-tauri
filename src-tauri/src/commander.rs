use crate::image;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn get_post(page: i8, limit:i8, tags: String, mode: String) -> image::ApiResponse {
  match image::get_post(page, limit, tags, mode).await {
    Ok(data) => image::ApiResponse {
      data: Some(data),
      code: 0,
    },
    Err(e) => {
      println!("get_post error {:?}", e);
      image::ApiResponse {
        data: None,
        code: 1,
      }
    }
  }
}

#[tauri::command]
pub async fn download_image(app_handle: AppHandle, url: String) {
  let mut progress = image::Progress::new(url.clone(), 0, 0, app_handle);
  if (image::download_image_progress_strut(url, &mut progress).await).is_err() {
    progress.error();
  };
}

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  // Show main window
  window.get_window("main").unwrap().show().unwrap();
}
