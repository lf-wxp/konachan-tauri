use crate::image;
use tauri::{command, AppHandle, Manager, Window};

#[command]
pub async fn get_post(page: u32, limit: u8, tags: String, mode: String) -> image::ApiResponse {
  println!("get_post {:?}", page);
  match image::get_post(page, limit, tags, mode).await {
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
  let mut progress = image::Progress::new(url.clone(), 0, 0, app_handle);
  if (image::download_image_progress_strut(url, &mut progress).await).is_err() {
    progress.error();
  };
}

#[command]
pub async fn close_splashscreen(window: Window) {
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  window.get_window("main").unwrap().show().unwrap();
}
