pub fn endpoint_name() -> Result<String, Box<dyn Error>> {
    let mut request = ureq::request(&method, &url);

    request = request.set(&header_name, &header_value);

    // request.call() // If there is no body
    // request.send_string() // If there is a body
}
