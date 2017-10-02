
use std::default::Default;


pub enum ErrorCode {
    Int(i32),
    Str(String)
}
impl From<i32> for ErrorCode {
    fn from(v: i32) -> Self {
        ErrorCode::Int(v)
    }
}
impl From<String> for ErrorCode {
    fn from(v: String) -> Self {
        ErrorCode::Str(v)
    }
}

pub struct WeChatError {
    // 错误代码
    pub errcode: ErrorCode,
    // 错误代码描述
    pub errmsg: String,
}

pub struct WeChatClientError {
    pub base: WeChatError,
    client: (),
    request: (),
    response: (),
}
impl WeChatClientError {
    pub fn new(
        errcode: ErrorCode, errmsg: String,
        client: (),
        request: (),
        response: (),
    ) -> Self {
        WeChatClientError {
            base: WeChatError{ errcode, errmsg },
            client, request, response,
        }
    }
}

pub struct WeChatPayError {
    pub base: WeChatClientError,
    // 返回状态码
    return_code: String,
    // 返回信息
    return_msg: String,
    // 业务结果
    result_code: String,
}
impl WeChatPayError {
    pub fn new(
        errcode: ErrorCode,
        errmsg: String,
        return_code: String,
        return_msg: String,
        result_code: String,
        client: (),
        request: (),
        response: (),
    ) -> Self {
        let base = WeChatClientError::new(
            errcode, errmsg,
            client, request, response
        );
        WeChatPayError {
            base,
            return_code,
            return_msg,
            result_code,
        }
    }
}

pub struct InvalidSignatureError {
    pub base: WeChatError,
}
impl Default for InvalidSignatureError {
    fn default() -> Self {
        InvalidSignatureError {
            base: WeChatError {
                errcode: ErrorCode::from(-40001),
                errmsg: format!("Invalid signature"),
            }
        }
    }
}

pub struct APILimitedError {
    pub base: WeChatClientError
}

pub struct InvalidAppIdError {
    pub base: WeChatError,
}
impl Default for InvalidAppIdError {
    fn default() -> Self {
        InvalidAppIdError {
            base: WeChatError {
                errcode: ErrorCode::from(-40005),
                errmsg: format!("Invalid AppId"),
            }
        }
    }
}

pub struct WeChatOAuthError {
    pub base: WeChatClientError
}
