pub mod auth;
pub mod category;
pub mod pagination;
pub mod question;
pub mod quiz;
pub mod result;
pub mod submission;
pub mod user;

pub fn vec_stringify<T: ToString>(vec: Vec<T>) -> Vec<String> {
    let mut ret = Vec::new();
    for element in vec {
        ret.push(element.to_string());
    }
    ret
}
