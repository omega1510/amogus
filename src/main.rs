use curl::easy::{Easy, List};
use serde_json::Value;
use std::str;

/// Retrieve the code from the amogus channel description
///
fn get_code() -> Value {
    // Create a new handle using the libcurl easy interface
    //
    let mut request = Easy::new();
    request.url("https://youtube.googleapis.com/youtube/v3/channels?part=snippet%2CcontentDetails%2Cstatistics&id=UCqUBqGZu_7OMAV6-THlcN4g&key=AIzaSyCa-pFNCGoVt_8Jd78yWGxf8sao5ov-SDE").unwrap();

    // Set the headers for the request
    // This allows us to set the content type to JSON
    //
    let mut list = List::new();
    list.append("Accept: application/json").unwrap();
    request.http_headers(list).unwrap();

    // Write the response to a string
    //
    let mut response: String = String::new();
    {
        let mut transfer = request.transfer();
        transfer
            .write_function(|data| {
                response.push_str(str::from_utf8(data).unwrap());
                Ok(data.len())
            })
            .unwrap();

        // Perform the request
        // This will return a Result<(), curl::Error>, so it must be unwrapped
        //
        transfer.perform().unwrap();
    }

    // Parse the JSON response
    //
    let response: Value = serde_json::from_str(response.as_str()).unwrap();
    response
}

/// Decode the code
///
fn decode(code: &Value) -> String {
    let mut decoded_code = String::new();

    // Offset to implement Caesar cipher
    //
    let offset = 26;

    for word in code.as_str().unwrap().split(' ') {
        for letter in word.split('/') {
            decoded_code.push_str(
                // Decode the letter
                //
                char::from_u32(offset - letter.parse::<u32>().unwrap() + 65)
                    .unwrap()
                    .to_string()
                    .as_str(),
            );
        }

        // Add a space between words
        //
        decoded_code.push(' ');
    }

    decoded_code
}

fn main() {
    let code = &get_code()["items"][0]["snippet"]["description"];
    println!("Code: {}", code);
    println!("Decoded: {}", decode(code));
}
