use std::collections::HashMap;

use serde_json::{json, Value};

use crate::fetcher;

// can expand this to be one function that could hold multiple sources??
pub async fn fetch_images(subreddit: String, sort: String) -> String {
    let mut client = fetcher::Fetcher::default();
    client.setup();
    client.get_posts(subreddit, sort).await
}

// function that will get the raw json, extract the images and return them
pub fn image_parser(image_data: String) -> Value {
    // convert the data into a JSON format
    let parsed_response: Value =
        serde_json::from_str(&image_data).expect("there was an error converting response to json");

    return parsed_response;
}

pub fn extract_images(parsed_images: Value) -> Value {
    // get data from HTTP JSON response
    let images = parsed_images["data"]["children"].as_array().unwrap();

    let mut extracted: Vec<HashMap<&str, &Value>> = vec![];

    // loop over each reddit post
    for image in images.iter() {
        //create Map to store collected data
        let mut img_item = HashMap::new();

        let img_data = &image["data"].as_object().unwrap();

        // add needed data from post into Map
        img_item.insert("id", &img_data["id"]);
        img_item.insert("title", &img_data["title"]);
        img_item.insert("url", &img_data["url"]);
        img_item.insert("author", &img_data["author"]);

        //add the map to vector
        extracted.push(img_item);
    }

    //convert the map into JSON and return to frontend
    return json!(extracted);
}

fn get_image_info() {}
