use crate::image;

#[tauri::command]
pub fn get_post(page: i8) -> image::Post {
  println!("the page is {}", page);
  image::get_post(page).unwrap()
}
