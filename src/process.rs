use crate::responses::UserErrorMessage;

pub fn process_request(id: u32) -> String {
    if id != 0 {
        std::fmt::format(format_args!("processed request for id: {:?}", id))
    } else {
        std::fmt::format(format_args!("error: id must be integer greater than 0"))
    }
}
