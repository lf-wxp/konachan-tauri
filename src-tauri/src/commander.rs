use crate::image::{self, ApiMode};
use tauri::{command, AppHandle, Manager, Window};

#[command]
pub async fn get_post(page: u32, limit: u8, tags: String, mode: String) -> image::ApiResponse {
  println!("get_post {:?}", page);
  let api_mode = match mode.as_str() {
    "json" => ApiMode::Json,
    _ => ApiMode::Xml,
  };
  match image::get_post(page, limit, tags, api_mode).await {
    Ok(data) => image::ApiResponse {
      data: Some(data),
      code: 0,
      msg: None,
    },
    Err(e) => {
      println!("get_post error {:?}", e);
      image::ApiResponse {
        data: None,
        code: 1,
        msg: None,
      }
    }
  }
}

#[command]
pub async fn download_image(app_handle: AppHandle, url: String) {
  println!("download {:?}", url);
  let download_dir = match app_handle.path().download_dir() {
    Ok(dir) => dir,
    Err(e) => {
      eprintln!("failed to get download directory: {}", e);
      return;
    }
  };
  let mut progress = image::Progress::new(url.clone(), 0, 0, app_handle);
  if image::download_image_progress_strut(url, &mut progress, download_dir)
    .await
    .is_err()
  {
    progress.error();
  }
}

 #[command]
 pub fn close_splashscreen(window: Window) -> Result<(), String> {
   if let Some(splashscreen) = window.get_webview_window("splashscreen") {
     splashscreen.close().map_err(|e| e.to_string())?;
   }
   window.get_webview_window("main")
     .ok_or("main window not found")?
     .show()
     .map_err(|e| e.to_string())?;
   Ok(())
 }
