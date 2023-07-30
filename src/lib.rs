#[path = "./res_types.rs"]
mod res_types;
#[path = "./res_std_messages.rs"]
mod res_std_messages;
#[path = "./res_message.rs"]
mod res_message;
#[path = "./net_status_code.rs"]
mod net_status_code;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
