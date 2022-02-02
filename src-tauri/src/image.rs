use bytes::Bytes;
use reqwest;
use roxmltree;
use serde::{Serialize, Deserialize};

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Image {
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
    safe: bool,
  }

  #[derive(Debug, Serialize)]
  pub struct ApiResponse {
    pub data: Option<Post>,
    pub code: u8,
  }
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Post {
    count: i32,
    images: Vec<Image>,
  }

  // pub type PostResult = Result<Post, Box<dyn std::error::Error>>;
  pub type PostResult = Result<Post, reqwest::Error>;

  pub struct ImageResponse {
    pub data: Bytes,
  }
  pub const API: &str = "https://konachan.net/post.xml";

  pub fn get_image(url: String) -> Result<Bytes, reqwest::Error> {
    reqwest::blocking::get(url)?.bytes()
  }

  pub fn attr_to_int(e: roxmltree::Node, attr: &str) -> i32 {
    e.attribute(attr)
      .unwrap()
      .to_string()
      .parse::<i32>()
      .unwrap()
  }

  pub fn get_post(page: i8) -> PostResult {
    let client = reqwest::blocking::Client::new();
    let resp = client
      .get(API)
      .query(&[("page", page)])
      .send()?
      .text()?;
    Ok(parse(resp.to_string()))
  }

  pub fn parse(xml: String) -> Post {
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let elem = doc.descendants();
    let mut count = 0;
    let mut images: Vec<Image> = vec![];
    for e in elem {
      match e.tag_name().name() {
        "posts" => {
          count = attr_to_int(e, "count");
        }
        "post" => {
          images.push(Image {
            url: e.attribute("file_url").unwrap().to_string(),
            width: attr_to_int(e, "width"),
            height: attr_to_int(e, "height"),
            preview: e.attribute("preview_url").unwrap().to_string(),
            preview_width: attr_to_int(e, "preview_width"),
            preview_height: attr_to_int(e, "preview_height"),
            sample: e.attribute("sample_url").unwrap().to_string(),
            sample_width: attr_to_int(e, "sample_width"),
            sample_height: attr_to_int(e, "sample_height"),
            tags: e.attribute("tags").unwrap().to_string(),
            safe: e.attribute("rating").unwrap() == "s",
          });
        }
        _ => {}
      }
    }
    Post { count, images }
  }
