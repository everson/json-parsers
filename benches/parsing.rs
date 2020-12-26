
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct RequestAttributes {
  pub values: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawDevice {
  ifa: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawAppPublisherExtRp {
  account_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawAppPublisherExt {
  rp: Option<RawAppPublisherExtRp>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawAppPublisher {
  ext: Option<RawAppPublisherExt>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawAppExtRp {
  site_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawAppExt {
  rp: Option<RawAppExtRp>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawApp {
  bundle: Option<String>,
  publisher: Option<RawAppPublisher>,
  ext: Option<RawAppExt>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawBidRequest {
  id: Option<String>,
  device: Option<RawDevice>,
  app: Option<RawApp>,
}

fn parse_raw(s: String) -> RawBidRequest {
  let rbr = serde_json::from_str(&s).unwrap();
  rbr
}

pub fn parse(s: String) -> RequestAttributes {
  let rbr = parse_raw(s);

  let mut ra = RequestAttributes::default();

  rbr
    .device
    .map(|d| d.ifa.map(|d_id| ra.values.insert("deviceId".into(), d_id)));

  ra
}
