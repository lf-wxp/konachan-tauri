use crate::image;
use tauri::AppHandle; 

#[tauri::command]
pub async fn get_post(page: i8) -> image::ApiResponse {
  match image::get_post(page).await {
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
pub async fn download_image(app_handle: AppHandle, url: String) -> () {
  let mut progress = image::Progress::new(url.clone(), 0, 0, app_handle);
  if let Err(_) = image::download_image_progress_strut(url, &mut progress).await {
    progress.error();
  };
}
