use axum::{
  body::BodyDataStream, 
  extract::Query, 
  // extract::TypedHeader,
  // headers::ContentType,
  http::StatusCode, 
  response::Redirect, 
  routing::{get, post}, 
  Router};
use serde::{Deserialize};

#[derive(Deserialize, Default)]
struct FromKey {
  from: String,
  key: String
}
#[derive(Deserialize, Default)]
struct FromTo {
  from: String,
  to: String
}
// day5
#[derive(Deserialize, Default, Debug)]
struct SendPackageData {
  package: Package,
}
#[derive(Deserialize, Default, Debug)]
struct Package {
  _name: String,
  _authors: Vec<String>,
  _keywords: Vec<String>,
  metadata: Option<Metadatas>,
}
#[derive(Debug, Deserialize)]
struct Metadatas {
  orders: Vec<Metadata>,
}
// #[derive(Debug, Deserialize)]
// #[serde(untagged)] // 自動的に型を推測
// enum Input {
//     Number(u32),
//     Text(String),
// }
#[derive(Deserialize, Default, Debug)]
struct Metadata {
  item: String,
  #[serde(deserialize_with = "deserialize_quantity")]
  quantity: String
}

fn deserialize_quantity<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: serde::Deserializer<'de>,
{
  use serde::de::{self, Visitor};
  use std::fmt;
  struct QuantityVisitor;
  impl<'de> Visitor<'de> for QuantityVisitor {
    type Value = String;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a string or an integer")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v.to_string())
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v.to_string())
    }
  }
  deserializer.deserialize_any(QuantityVisitor)
}

async fn hello_world() -> &'static str {
  "Hello, bird!"
}
async fn redirect() -> (StatusCode, Redirect) {
  (
    StatusCode::FOUND, 
    Redirect::to("https://www.youtube.com/watch?v=9Gc4QTqslN4")
  )
}
async fn calcu_ip(Query(params): Query<FromKey>) -> String {
  let from: Vec<u16> = params.from.split('.').map(|x| x.parse::<u16>().unwrap_or(0)).collect();
  let key: Vec<u16> = params.key.split('.').map(|x| x.parse::<u16>().unwrap_or(0)).collect();
  let dest: Vec<u16> = from.into_iter().zip(key.into_iter()).map(|(f, k)| (f+k)%256).collect();
  let return_dest = format!("{}.{}.{}.{}", dest[0], dest[1], dest[2], dest[3]);
  return_dest
}
async fn calcu_key(Query(params): Query<FromTo>) -> String {
  let from: Vec<u16> = params.from.split('.').map(|x| x.parse::<u16>().unwrap_or(0)).collect();
  let to: Vec<u16> = params.to.split('.').map(|x| x.parse::<u16>().unwrap_or(0)).collect();
  let key: Vec<u16> = from.into_iter().zip(to.into_iter()).map(|(f, t)| (t+256-f)%256).collect();
  let return_key = format!("{}.{}.{}.{}", key[0], key[1], key[2], key[3]);
  return_key
}
async fn calcuv6_ip(Query(params): Query<FromKey>) -> String {
  let from: Vec<&str> = params.from.split(':').collect();
  let key: Vec<&str> = params.key.split(':').collect();
  // println!("from {:?}", &from);
  // println!("key {:?}", &key);

  let mut from_bit: Vec<u32> = vec![];
  let from_0_count = 9 - from.len();
  let mut key_bit: Vec<u32> = vec![];
  let key_0_count = 9 - key.len();
  
  for f in from {
    if f.is_empty() { for _i in 0..from_0_count { from_bit.push(0); } }
    else { from_bit.push( u32::from_str_radix(f, 16).unwrap()); }
  }
  for k in key {
    if k.is_empty() { for _i in 0..key_0_count { key_bit.push(0); } }
    else { key_bit.push(u32::from_str_radix(k, 16).unwrap()); }
  }
  // println!("from {:?}", &from_bit);
  // println!("key {:?}", &key_bit);

  let dest: Vec<u32> = from_bit.into_iter().zip(key_bit.into_iter()).map(|(f, k)| f^k).collect();
  let return_dest = format!("{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}", dest[0],dest[1],dest[2],dest[3],dest[4],dest[5],dest[6],dest[7]);
  return_dest
}
async fn calcuv6_key(Query(params): Query<FromTo>) -> String {
  let from: Vec<&str> = params.from.split(':').collect();
  let to: Vec<&str> = params.to.split(':').collect();
  // let key: Vec<u16> = from.into_iter().zip(to.into_iter()).map(|(f, t)| (t+256-f)%256).collect();
  let mut from_bit: Vec<u32> = vec![];
  let from_0_count = 9 - from.len();
  let mut to_bit: Vec<u32> = vec![];
  let to_0_count = 9 - to.len();
  for f in from {
    if f.is_empty() { for _i in 0..from_0_count { from_bit.push(0); } } 
    else { from_bit.push(u32::from_str_radix(f, 16).unwrap()); }
  }
  for t in to {
    if t.is_empty() { for _i in 0..to_0_count { to_bit.push(0); } }
    else { to_bit.push(u32::from_str_radix(t, 16).unwrap()) }
  }
  // println!("from {:?}", &from_bit);
  // println!("to {:?}", &to_bit);

  let key: Vec<u32> = from_bit.into_iter().zip(to_bit.into_iter()).map(|(f, t)| f^t).collect();
  let return_key = format!("{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}", key[0],key[1],key[2],key[3],key[4],key[5],key[6],key[7]);
  return_key
}
async fn manifest5(body: String) -> Result<String,StatusCode> {
  // let req: Value = body.clone().parse().unwrap();
  // println!("=\n{}\n=",&body);
  // println!("{:?}",req["package"]);
  // println!("{:?}",req["package"]["metadata"]);

  let p:SendPackageData = toml::from_str(&body).unwrap();

  // p.package.metadata.map(|data| println!("{:?}",data.orders));
  let mut return_data = "".to_string();
  match p.package.metadata {
    Some(data) => {
      // let return_data = data.orders.into_iter().map(|d| println!("{:?}",d));
      for d in data.orders {
        let mut qua = d.quantity;
        if qua.parse::<u32>().is_ok(){
          return_data += &format!("{}: {}\n",d.item,qua);
        }
      }
      return_data.pop();
      Ok(return_data)
    },
    None => Err(StatusCode::NO_CONTENT)
  }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/seek", get(redirect))
    .route("/-1/seek2", get(|| async { Redirect::permanent("https://www.youtube.com/watch?v=9Gc4QTqslN4") }))
    .route("/2/dest" ,get(calcu_ip))
    .route("/2/key" ,get(calcu_key))
    .route("/2/v6/dest" ,get(calcuv6_ip))
    .route("/2/v6/key" ,get(calcuv6_key))
    .route("/5/manifest", post(manifest5));

  Ok(router.into())
}
