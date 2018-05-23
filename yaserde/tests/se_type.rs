#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::Write;
use yaserde::YaSerialize;
use yaserde::ser::to_string;

macro_rules! convert_and_validate {
  ($type: ty, $value: expr, $content: expr) => {{
    #[derive(YaSerialize, PartialEq, Debug)]
    #[yaserde(root = "data")]
    pub struct Data {
      item: $type,
    }
    let model = Data { item: $value };

    let data: Result<String, String> = to_string(&model);
    let content = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?><data><item>") + $content
      + "</item></data>";
    assert_eq!(data, Ok(content));
  }};
}

#[test]
fn ser_type() {
  convert_and_validate!(bool, true, "true");
  convert_and_validate!(u8, 12 as u8, "12");
  convert_and_validate!(i8, 12 as i8, "12");
  convert_and_validate!(i8, -12 as i8, "-12");
  convert_and_validate!(u16, 12 as u16, "12");
  convert_and_validate!(i16, 12 as i16, "12");
  convert_and_validate!(i16, -12 as i16, "-12");
  convert_and_validate!(u32, 12 as u32, "12");
  convert_and_validate!(i32, 12 as i32, "12");
  convert_and_validate!(i32, -12 as i32, "-12");
  convert_and_validate!(u64, 12 as u64, "12");
  convert_and_validate!(i64, 12 as i64, "12");
  convert_and_validate!(i64, -12 as i64, "-12");
}
