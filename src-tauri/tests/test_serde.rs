use serde::{Deserialize, Serialize};

#[test]
fn test_serde() {
    let data = r#"{"resultCode":"1"}"#;
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "camelCase"))]
    pub struct EmptyData;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "camelCase"))]
    pub struct Response<T> {
        pub result_code: String,
        pub data: Option<T>,
    }

    let mut resp = serde_json::from_str::<Response<EmptyData>>(data).unwrap();
    let data = resp.data.take();
    if data.is_none() {
        println!("no data in");
    } else {
        println!("{:?}", data);
    }
}
