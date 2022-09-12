use futures_util::StreamExt;
use reqwest;
use roxmltree;
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::{self, Cursor, Write};
use std::path::Path;
use std::time::Duration;
use tauri::api::path::download_dir;
use tauri::Manager;
use urlencoding::decode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
  id: i32,
  url: String,
  width: i32,
  height: i32,
  preview: String,
  preview_width: i32,
  preview_height: i32,
  sample: String,
  sample_width: i32,
  sample_height: i32,
  tags: String,
  security: bool,
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
  pub data: Option<Post>,
  pub code: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
  count: i32,
  images: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressPayload {
  percent: f64,
  url: String,
  status: String,
}

pub struct Progress {
  url: String,
  total: u64,
  receive: u64,
  app: tauri::AppHandle,
}

impl Progress {
  pub fn new(url: String, total: u64, receive: u64, app: tauri::AppHandle) -> Progress {
    Progress {
      url,
      total,
      receive,
      app,
    }
  }

  fn set_receive(&mut self, size: u64) -> () {
    self.receive = size;
  }

  fn set_total(&mut self, size: u64) -> () {
    self.total = size;
  }

  fn get_percent(&self) -> f64 {
    (self.receive as f64 / self.total as f64) as f64
  }

  fn update(&self) -> () {
    let percent = self.get_percent();
    let status = if percent == 1 as f64 {
      "success"
    } else {
      "pending"
    };
    let payload = ProgressPayload {
      percent,
      url: self.url.clone(),
      status: status.to_string(),
    };
    let _ = &self.app.emit_all("progress", payload);
  }

  pub fn error(&self) -> () {
    let payload = ProgressPayload {
      percent: self.get_percent(),
      url: self.url.clone(),
      status: "fail".to_string(),
    };
    let _ = &self.app.emit_all("progress", payload);
  }
}

pub type ResultDyn<T> = Result<T, Box<dyn Error>>;

pub const API_XML: &str = "https://konachan.net/post.xml";
pub const API_JSON: &str = "https://acglife.club/api/post";

pub fn get_file_name(url: &str) -> Option<&str> {
  let name = Path::new(url).file_name()?.to_str()?;
  Some(name)
}

pub fn create_file(url: String) -> ResultDyn<File> {
  let file_name = get_file_name(&url).ok_or("get file name error")?;
  let mut download_path = download_dir().ok_or("get download dir error")?;
  download_path.push(file_name);
  let file = File::create(download_path.as_path())?;
  Ok(file)
}

pub async fn download_image_progress_strut(url: String, progress: &mut Progress) -> ResultDyn<()> {
  let res = reqwest::get(&url).await?;
  let total = res
    .content_length()
    .ok_or(format!("Failed to get content length {}", &url))?;
  progress.set_total(total);
  let mut stream = res.bytes_stream();
  let mut file = create_file(url)?;
  let mut downloaded: u64 = 0;
  while let Some(item) = stream.next().await {
    let chunk = item.or(Err(format!("Error while downloading file")))?;
    file
      .write(&chunk)
      .or(Err(format!("Error while writing to file")))?;
    let new = min(downloaded + (chunk.len() as u64), total);
    downloaded = new;
    progress.set_receive(new);
    progress.update();
  }
  Ok(())
}

#[allow(dead_code)]
pub async fn download_image(url: String) -> Result<(), Box<dyn Error>> {
  let res = reqwest::get(&url).await?;
  let mut file = create_file(url)?;
  let mut content = Cursor::new(res.bytes().await?);
  io::copy(&mut content, &mut file)?;
  Ok(())
}

pub fn attr_to_int(e: roxmltree::Node, attr: &str) -> i32 {
  e.attribute(attr)
    .unwrap_or("")
    .to_string()
    .parse::<i32>()
    .unwrap_or(0)
}
pub fn attr_to_string(e: roxmltree::Node, attr: &str) -> String {
  e.attribute(attr).unwrap_or("").to_string()
}

pub async fn get_post_json(page: i8, tags: String) -> ResultDyn<Post> {
  let client = reqwest::Client::new();
  let resp = client
    .get(API_JSON)
    .header("x-api-key", "konachan-api")
    .timeout(Duration::from_secs(20))
    .query(&[("page", page)])
    .query(&[("tags", tags)])
    .send()
    .await?
    .text()
    .await?;
  let data: ApiResponse = serde_json::from_str(&resp)?;
  let post = data.data.ok_or("get image data from json api error")?;
  Ok(post)
}

pub async fn get_post_xml(page: i8, tags: String) -> ResultDyn<Post> {
  let client = reqwest::Client::new();
  let resp = client
    .get(API_XML)
    .timeout(Duration::from_secs(10))
    .query(&[("page", page)])
    .query(&[("tags", tags)])
    .send()
    .await?
    .text()
    .await?;
  parse(resp.to_string())
}

pub async fn get_post(page: i8, tags: String, mode: String) -> ResultDyn<Post> {
  if mode == "json" {
    return get_post_json(page, tags).await;
  }
  get_post_xml(page, tags).await
}

pub fn parse(xml: String) -> ResultDyn<Post> {
  let doc = roxmltree::Document::parse(&xml)?;
  let elem = doc.descendants();
  let mut count = 0;
  let mut images: Vec<Image> = vec![];
  for e in elem {
    match e.tag_name().name() {
      "posts" => {
        count = attr_to_int(e, "count");
      }
      "post" => {
        let url = e.attribute("file_url").unwrap_or("");
        let encoded_name = url.split("/").last().unwrap_or("");
        let name = decode(encoded_name).map_or("".to_string(), |x| x.to_string());
        images.push(Image {
          id: attr_to_int(e, "id"),
          url: url.to_string(),
          width: attr_to_int(e, "width"),
          height: attr_to_int(e, "height"),
          preview: attr_to_string(e, "preview_url"),
          preview_width: attr_to_int(e, "preview_width"),
          preview_height: attr_to_int(e, "preview_height"),
          sample: attr_to_string(e, "sample_url"),
          sample_width: attr_to_int(e, "sample_width"),
          sample_height: attr_to_int(e, "sample_height"),
          tags: attr_to_string(e, "tags"),
          security: attr_to_string(e, "rating") == "s",
          name,
        });
      }
      _ => {}
    }
  }
  Ok(Post { count, images })
}
