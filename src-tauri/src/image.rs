use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
  cmp::min,
  fs::{self, File},
  io::{ErrorKind, Write},
  path::{Path, PathBuf},
  time::Duration,
};
use tauri::{AppHandle, Emitter};
use urlencoding::decode;

pub const API_XML: &str = "https://konachan.com/post.xml";
pub const API_JSON: &str = "https://pic.onlyxp.me/api/post";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ApiMode {
  Json,
  Xml,
}

impl AsRef<str> for ApiMode {
  fn as_ref(&self) -> &str {
    match self {
      ApiMode::Json => "json",
      ApiMode::Xml => "xml",
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Image {
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
  pub msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
  count: i32,
  images: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ProgressPayload {
  percent: f64,
  url: String,
  status: String,
}

pub struct Progress {
  url: String,
  total: u64,
  receive: u64,
  app: AppHandle,
}

impl Progress {
  pub fn new(url: String, total: u64, receive: u64, app: AppHandle) -> Progress {
    Progress {
      url,
      total,
      receive,
      app,
    }
  }

  fn set_receive(&mut self, size: u64) {
    self.receive = size;
  }

  fn set_total(&mut self, size: u64) {
    self.total = size;
  }

  /// Reports a finished transfer. Only called once the temp file has been
  /// renamed, so `loaded` always means "the image is on disk under its final
  /// name".
  fn mark_complete(&mut self) {
    self.receive = self.total;
    self.update();
  }

  fn get_percent(&self) -> f64 {
    // A zero total would make the ratio `NaN`, which `serde_json` serialises as
    // `null` and leaves the UI with no usable value.
    if self.total == 0 {
      return 0_f64;
    }
    self.receive as f64 / self.total as f64
  }

  fn update(&self) {
    let percent = self.get_percent();
    let status = if percent == 1_f64 {
      "loaded"
    } else {
      "pending"
    };
    let payload = ProgressPayload {
      percent,
      url: self.url.clone(),
      status: status.to_string(),
    };
    let _ = self.app.emit("progress", payload);
  }

  pub fn error(&self) {
    let payload = ProgressPayload {
      percent: self.get_percent(),
      url: self.url.clone(),
      status: "error".to_string(),
    };
    let _ = self.app.emit("progress", payload);
  }
}

pub fn get_file_name(url: &str) -> Result<String> {
  let name = Path::new(url)
    .file_name()
    .ok_or_else(|| anyhow!("failed to extract file name from url"))?
    .to_str()
    .ok_or_else(|| anyhow!("failed to convert file name to string"))?;
  // The percent-decoding happens *after* `file_name()`, so an encoded
  // separator survives it: `../..%2Fevil.png` is a single path component
  // before decoding and `../../evil.png` after. The decoded value is therefore
  // untrusted and has to be validated as a bare file name.
  let name = decode(name)
    .map_err(|e| anyhow!("failed to percent-decode the file name `{name}`: {e}"))?
    .into_owned();
  ensure_bare_file_name(&name)?;
  Ok(name)
}

/// Rejects any decoded name that could point outside of the download directory.
fn ensure_bare_file_name(name: &str) -> Result<()> {
  let is_bare = !name.is_empty()
    && name != "."
    && name != ".."
    && !name.contains(['/', '\\', '\0'])
    && Path::new(name).components().count() == 1;
  if !is_bare {
    return Err(anyhow!("refusing to use `{name}` as a download file name"));
  }
  Ok(())
}

pub fn get_file_full_path(file_name: String, download_dir: PathBuf) -> Result<PathBuf> {
  let mut download_path = download_dir;
  download_path.push(file_name);
  Ok(download_path.as_path().to_owned())
}

pub fn get_tmp_file_full_path(file_name: String, download_dir: PathBuf) -> Result<PathBuf> {
  let mut name = file_name;
  name.push_str(".tmp");
  get_file_full_path(name, download_dir)
}

pub fn is_file_exist(file_name: String, download_dir: PathBuf) -> Result<bool> {
  let path = get_file_full_path(file_name, download_dir)?;
  Ok(Path::new(&path).exists())
}

pub fn rename_tmp_file(file_name: String, download_dir: PathBuf) -> Result<()> {
  fs::rename(
    get_tmp_file_full_path(file_name.clone(), download_dir.clone())?,
    get_file_full_path(file_name, download_dir)?,
  )?;
  Ok(())
}

// Timeouts for the whole transfer. Without the upper bound a connection that
// stalls after the headers would keep the task, and its progress entry, alive
// forever.
const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(300);
const DOWNLOAD_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn download_image_progress_strut(
  url: String,
  progress: &mut Progress,
  download_dir: PathBuf,
) -> Result<()> {
  let file_name = get_file_name(&url)?;
  if is_file_exist(file_name.clone(), download_dir.clone())? {
    progress.set_total(1);
    progress.set_receive(1);
    progress.update();
    return Ok(());
  }

  let tmp_path = get_tmp_file_full_path(file_name.clone(), download_dir.clone())?;
  // Every failure path drops the partial file: there is no resume support, so a
  // leftover `.tmp` is indistinguishable from a download that finished.
  if let Err(error) = transfer(&url, progress, &tmp_path).await {
    remove_partial_file(&tmp_path);
    return Err(error);
  }

  // `transfer` closed the file handle with its own scope; renaming an open file
  // fails on Windows.
  rename_tmp_file(file_name, download_dir)?;
  progress.mark_complete();
  Ok(())
}

/// Streams `url` into `tmp_path`, reporting progress as it goes.
///
/// Returns `Ok` only when every byte announced by `Content-Length` was written:
/// a rejected HTTP status or a short body is a failure, not a download.
async fn transfer(url: &str, progress: &mut Progress, tmp_path: &Path) -> Result<()> {
  let res = reqwest::Client::builder()
    .connect_timeout(DOWNLOAD_CONNECT_TIMEOUT)
    .timeout(DOWNLOAD_TIMEOUT)
    .build()
    .map_err(|e| anyhow!("failed to build the download client: {e}"))?
    .get(url)
    .send()
    .await
    .map_err(|e| anyhow!("error while downloading {url}: {e}"))?
    // Without this an error page (404/403 body) would be saved as the image.
    .error_for_status()
    .map_err(|e| anyhow!("download rejected for {url}: {e}"))?;

  let total = res
    .content_length()
    .ok_or_else(|| anyhow!("failed to get content length: {url}"))?;
  if total == 0 {
    return Err(anyhow!("server advertised an empty body for {url}"));
  }
  progress.set_total(total);

  let mut file =
    File::create(tmp_path).map_err(|e| anyhow!("failed to create {}: {e}", tmp_path.display()))?;
  let mut stream = res.bytes_stream();
  let mut downloaded: u64 = 0;
  while let Some(item) = stream.next().await {
    let chunk = item.map_err(|e| anyhow!("error while downloading {url}: {e}"))?;
    file
      .write_all(&chunk)
      .map_err(|e| anyhow!("error while writing to {}: {e}", tmp_path.display()))?;
    downloaded += chunk.len() as u64;
    progress.set_receive(min(downloaded, total));
    // Completion is reported by the caller after the rename, never from inside
    // the loop: the stream can yield a final zero-length chunk, and reporting
    // `loaded` here renamed the temp file twice, failing with
    // `No such file or directory` after the image had already been saved.
    if downloaded < total {
      progress.update();
    }
  }

  if downloaded < total {
    return Err(anyhow!(
      "download of {url} was truncated: {downloaded} of {total} bytes received"
    ));
  }
  Ok(())
}

fn remove_partial_file(tmp_path: &Path) {
  match fs::remove_file(tmp_path) {
    Ok(()) => {}
    Err(e) if e.kind() == ErrorKind::NotFound => {}
    Err(e) => eprintln!(
      "failed to remove the partial download {}: {e}",
      tmp_path.display()
    ),
  }
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

pub async fn get_post_json(page: u32, limit: u8, tags: String) -> Result<Post> {
  let client = reqwest::Client::new();
  let resp = client
    .get(API_JSON)
    .header("x-api-key", "konachan-api")
    .timeout(Duration::from_secs(20))
    .query(&[("page", page)])
    .query(&[("tags", tags)])
    .query(&[("limit", limit)])
    .send()
    .await?
    .text()
    .await?;
  let data: ApiResponse = serde_json::from_str(&resp)?;
  let post = data
    .data
    .ok_or(anyhow!("get image data from json api error"))?;
  Ok(post)
}

pub async fn get_post_xml(page: u32, limit: u8, tags: String) -> Result<Post> {
  let client = reqwest::Client::new();
  let resp = client
    .get(API_XML)
    .timeout(Duration::from_secs(10))
    .query(&[("page", page)])
    .query(&[("tags", tags)])
    .query(&[("limit", limit)])
    .send()
    .await?
    .text()
    .await?;
  parse(resp)
}

pub async fn get_post(page: u32, limit: u8, tags: String, mode: ApiMode) -> Result<Post> {
  match mode {
    ApiMode::Json => get_post_json(page, limit, tags).await,
    ApiMode::Xml => get_post_xml(page, limit, tags).await,
  }
}

pub fn parse(xml: String) -> Result<Post> {
  let doc = roxmltree::Document::parse(&xml)?;
  let elem = doc.descendants();
  let mut count = 0;
  let mut images: Vec<Image> = Vec::with_capacity(50);
  for e in elem {
    match e.tag_name().name() {
      "posts" => {
        count = attr_to_int(e, "count");
      }
      "post" => {
        let url = e.attribute("file_url").unwrap_or("");
        let encoded_name = url.split('/').next_back().unwrap_or("");
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

#[cfg(test)]
mod tests {
  use super::get_file_name;

  #[test]
  fn file_name_is_percent_decoded() {
    assert_eq!(
      get_file_name("https://konachan.com/data/abc%20def.jpg").unwrap(),
      "abc def.jpg"
    );
    assert_eq!(
      get_file_name("https://konachan.com/data/%E8%93%9D%E5%A4%A9.png").unwrap(),
      "蓝天.png"
    );
  }

  #[test]
  fn plain_url_keeps_its_name() {
    assert_eq!(
      get_file_name("https://konachan.com/data/00f9a4b2e1c6.jpg").unwrap(),
      "00f9a4b2e1c6.jpg"
    );
  }

  #[test]
  fn encoded_separators_are_rejected() {
    // `file_name()` runs before the decoding, so these look like a single path
    // component and only turn into a traversal once decoded.
    for url in [
      "https://konachan.com/data/..%2F..%2Fevil.sh",
      "https://konachan.com/data/a%2Fb.png",
      "https://konachan.com/data/a%5Cb.png",
      "https://konachan.com/data/a%00b.png",
    ] {
      assert!(get_file_name(url).is_err(), "{url} must be rejected");
    }
  }

  #[test]
  fn empty_and_dot_names_are_rejected() {
    for url in [
      "/",
      "https://konachan.com/data/..",
      "https://konachan.com/data/%2E%2E",
    ] {
      assert!(get_file_name(url).is_err(), "{url} must be rejected");
    }
  }
}
