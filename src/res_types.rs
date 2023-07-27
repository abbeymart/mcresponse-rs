use std::collections::HashMap;
use crate::net_status_code::{NetCode};

pub struct ResponseMessage<T> {
    pub code: String,
    // Standard HttpCode
    pub res_code: NetCode,
    // Standard HttpCode Text
    pub res_message: String,
    pub message: String,
    pub value: T,
}

pub struct ResponseMessageOptions<T> {
    pub message: String,
    pub value: T,
}

pub type MessageParam<T> = HashMap<String, ResponseMessage<T>>;

