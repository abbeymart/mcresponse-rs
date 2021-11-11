use crate::net_status_code::NetCodes;
use crate::res_std_messages::std_res_messages;
use crate::res_types::*;

pub fn msg_func<T>(code: String, res_code: NetCodes, res_message: String, message: String, value: T) -> ResponseMessage<T> {
    ResponseMessage{
        code,
        res_code,
        res_message,
        message,
        value,
    }
}

pub fn get_res_message<T>(msg_type: String, options: ResponseMessageOptions<T>) -> ResponseMessage<T> {
    let mut value = None;
    let mut code = "unknown".to_string();
    let mut res_code = NetCodes::UnprocessableEntity;
    let mut res_message = "Unprocessable Entity".to_string();
    let mut message = "Unknown/Unspecified response message".to_string();

    let std_messages: MessageParam<T> = std_res_messages();
    // compose response-message
    match std_messages.get(&msg_type) {
        Some(&val) => {
            code = val.code;
            res_code = val.res_code;
            res_message = val.res_message;
            message = val.message;
            value = val.value;
            // update msgType and option-values: Message && Value
            if msg_type != "" {
                // set value to msgType, as specified
                code = msg_type;
            }
            if options.value != None {
                // set value to optional value
                value = options.value;
            }
            if options.message != "" {
                // append optional message
                message = format!("{}{}{}", message, " | ", options.message);
            }
        },
        None => {
            match std_messages.get("unknown") {
                Some(&val) => {
                    code = val.code;
                    res_code = val.res_code;
                    res_message = val.res_message;
                    message = val.message;
                    value = val.value;
                    // update msgType and option-values: Message && Value
                    if msg_type != "" {
                        // set value to msgType, as specified
                        code = msg_type;
                    }
                    if options.value != None {
                        // set value to optional value
                        value = options.value;
                    }
                    if options.message != "" {
                        // append optional message
                        message = format!("{}{}{}", message, " | ", options.message);
                    }
                },
                None => {}
            }
        }
    }

    msg_func(code, res_code, res_message, message, value)
}
