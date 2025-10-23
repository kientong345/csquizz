pub fn vec_stringify<T: ToString>(vec: Vec<T>) -> Vec<String> {
    let mut ret = Vec::new();
    for element in vec {
        ret.push(element.to_string());
    }
    ret
}

pub fn validate_email_name(email: &str) -> Result<(), String> {
    Ok(())
}
