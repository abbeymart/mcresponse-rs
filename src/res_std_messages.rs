use std::collections::HashMap;
use crate::net_status_code::{compute_status_text, NetCode};
use crate::res_types::*;


// Shared Status_text for NetCodes
pub static STATUS_TEXT: HashMap<NetCode, String> = compute_status_text();

pub fn std_res_messages<T>() -> MessageParam<T> {
    let mut res_messages_result= MessageParam::new();
    // let STATUS_TEXT = compute_status_text();

    res_messages_result.insert("paramsError".to_string(), ResponseMessage {
        code: "paramsError".to_string(),
        res_code: NetCode::NotAcceptable,
        res_message: STATUS_TEXT[NetCode::NotAcceptable],
        message: "Parameters checking error".to_string(),
        value: None,
    });

    res_messages_result.insert("checkError".to_string(), ResponseMessage {
        code: "paramsError".to_string(),
        res_code: NetCode::NotAcceptable,
        res_message: STATUS_TEXT[NetCode::NotAcceptable],
        message: "Parameters checking error".to_string(),
        value: None,
    });

    res_messages_result.insert("connectError".to_string(), ResponseMessage {
        code: "connectError".to_string(),
        res_code: NetCode::NetworkAuthenticationRequired,
        res_message: STATUS_TEXT[NetCode::NetworkAuthenticationRequired],
        message: "Connection error".to_string(),
        value: None,
    });

    res_messages_result.insert("validateError".to_string(), ResponseMessage {
        code: "paramsError".to_string(),
        res_code: NetCode::NotAcceptable,
        res_message: STATUS_TEXT[NetCode::NotAcceptable],
        message: "Validation error for inputs parameters".to_string(),
        value: None,
    });

    res_messages_result.insert("tokenExpired".to_string(), ResponseMessage {
        code: "tokenExpired".to_string(),
        res_code: NetCode::Unauthorized,
        res_message: STATUS_TEXT[NetCode::Unauthorized],
        message: "Unauthorized. Token / Access-key has expired. Please login again".to_string(),
        value: None,
    });

    res_messages_result.insert("unAuthorized".to_string(), ResponseMessage {
        code: "unAuthorized".to_string(),
        res_code: NetCode::Unauthorized,
        res_message: STATUS_TEXT[NetCode::Unauthorized],
        message: "Unauthorised Action or Task".to_string(),
        value: None,
    });

    res_messages_result.insert("notFound".to_string(), ResponseMessage {
        code: "notFound".to_string(),
        res_code: NetCode::NotFound,
        res_message: STATUS_TEXT[NetCode::NotFound],
        message: "Requested information not found".to_string(),
        value: None,
    });

    res_messages_result.insert("success".to_string(), ResponseMessage {
        code: "success".to_string(),
        res_code: NetCode::OK,
        res_message: STATUS_TEXT[NetCode::OK],
        message: "Request completed successfully".to_string(),
        value: None,
    });

    res_messages_result.insert("removeDenied".to_string(), ResponseMessage {
        code: "removeDenied".to_string(),
        res_code: NetCode::Unauthorized,
        res_message: STATUS_TEXT[NetCode::Unauthorized],
        message: "Remove action/task denied/unauthorised".to_string(),
        value: None,
    });

    res_messages_result.insert("removeError".to_string(), ResponseMessage {
        code: "removeDenied".to_string(),
        res_code: NetCode::UnprocessableEntity,
        res_message: STATUS_TEXT[NetCode::UnprocessableEntity],
        message: "Error removing/deleting information, retry or contact system-admin".to_string(),
        value: None,
    });

    res_messages_result.insert("removed".to_string(), ResponseMessage {
        code: "removed".to_string(),
        res_code: NetCode::OK,
        res_message: STATUS_TEXT[NetCode::OK],
        message: "Record(s) deleted/removed successfully".to_string(),
        value: None,
    });

    res_messages_result.insert("deleted".to_string(), ResponseMessage {
        code: "removed".to_string(),
        res_code: NetCode::OK,
        res_message: STATUS_TEXT[NetCode::OK],
        message: "Record(s) deleted/removed successfully".to_string(),
        value: None,
    });

    res_messages_result.insert("subItems".to_string(), ResponseMessage {
        code: "subItems".to_string(),
        res_code: NetCode::UnprocessableEntity,
        res_message: STATUS_TEXT[NetCode::UnprocessableEntity],
        message: "Record includes sub-items, which must be removed first".to_string(),
        value: None,
    });

    res_messages_result.insert("duplicate".to_string(), ResponseMessage {
        code: "duplicate".to_string(),
        res_code: NetCode::UnprocessableEntity,
        res_message: STATUS_TEXT[NetCode::UnprocessableEntity],
        message: "Duplicate record exists".to_string(),
        value: None,
    });

    res_messages_result.insert("updated".to_string(), ResponseMessage {
        code: "updated".to_string(),
        res_code: NetCode::OK,
        res_message: STATUS_TEXT[NetCode::OK],
        message: "Information update action completed successfully".to_string(),
        value: None,
    });

    res_messages_result.insert("updateError".to_string(), ResponseMessage {
        code: "updateError".to_string(),
        res_code: NetCode::UnprocessableEntity,
        res_message: STATUS_TEXT[NetCode::UnprocessableEntity],
        message: "Error updating information/record(s)".to_string(),
        value: None,
    });

    res_messages_result.insert("updateDenied".to_string(), ResponseMessage {
        code: "updateDenied".to_string(),
        res_code: NetCode::Unauthorized,
        res_message: STATUS_TEXT[NetCode::Unauthorized],
        message: "Update action/task not authorized".to_string(),
        value: None,
    });

    res_messages_result.insert("inserted".to_string(), ResponseMessage {
        code: "inserted".to_string(),
        res_code: NetCode::OK,
        res_message: STATUS_TEXT[NetCode::OK],
        message: "Information/record(s) inserted/created successfully".to_string(),
        value: None,
    });

    res_messages_result.insert("insertError".to_string(), ResponseMessage {
        code: "insertError".to_string(),
        res_code: NetCode::UnprocessableEntity,
        res_message: STATUS_TEXT[NetCode::UnprocessableEntity],
        message: "Error inserting/creating new information/record".to_string(),
        value: None,
    });

    res_messages_result.insert("exists".to_string(), ResponseMessage {
        code: "exists".to_string(),
        res_code: NetCode::UnprocessableEntity,
        res_message: STATUS_TEXT[NetCode::UnprocessableEntity],
        message: "Document/record exists".to_string(),
        value: None,
    });

    res_messages_result.insert("unknown".to_string(), ResponseMessage {
        code: "unknown".to_string(),
        res_code: NetCode::UnprocessableEntity,
        res_message: STATUS_TEXT[NetCode::UnprocessableEntity],
        message: "Unspecified response/error Message".to_string(),
        value: None,
    });

    res_messages_result
}
