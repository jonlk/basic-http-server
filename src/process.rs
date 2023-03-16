use crate::responses::UserErrorMessage;

pub fn process_request(id: u32) -> String {
    std::fmt::format(format_args!("processed request for id: {:?}", id))
}
