use std::io::{Read};

use curl;
use curl::easy::{self, Easy};
use http::{self, Request, Response};
use uuid::Uuid;

/// [生成随机数算法]
///
/// 微信支付API接口协议中包含字段nonce_str，主要保证签名不可预测。
/// 我们推荐生成随机数算法如下：调用随机数函数生成，将得到的值转换为字符串。
pub fn get_nonce_str() -> String {
    Uuid::new_v4().simple().to_string()
}

pub enum Certificate {
    Pk12{ path: String, password: String },
    Pem{ cert: String, key: String, password: String },
}

pub fn http_get(
    request: Request<Vec<u8>>,
    cert: Option<Certificate>,
) -> http::Result<Response<Vec<u8>>> {
    http_request("GET", request, cert)
}

pub fn http_post(
    request: Request<Vec<u8>>,
    cert: Option<Certificate>,
) -> http::Result<Response<Vec<u8>>> {
    http_request("POST", request, cert)
}

pub fn http_request(
    method: &'static str,
    request: Request<Vec<u8>>,
    cert: Option<Certificate>,
) -> http::Result<Response<Vec<u8>>>
{
    let mut res_body = Vec::new();
    let mut raw_headers = Vec::new();
    let code: Result<u32, curl::Error> = {
        let mut handle = Easy::new();
        match cert {
            Some(Certificate::Pem{cert, key, password}) => {
                handle.ssl_cert(cert).unwrap();
                handle.ssl_key(key).unwrap();
                if password.len() > 0 {
                    handle.key_password(password.as_str()).unwrap();
                }
            }
            Some(Certificate::Pk12{..}) => {
                // TODO:
            }
            None => {}
        }
        handle.url(format!("{}", request.uri()).as_str()).unwrap();
        handle.custom_request(method).unwrap();
        let mut list = easy::List::new();
        for (key, value) in request.headers() {
            list.append(format!("{:?}: {:?}", key, value).as_str()).unwrap();
        }
        handle.http_headers(list).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.read_function(|buf| {
                Ok(request.body().as_slice().read(buf).unwrap_or(0))
            }).unwrap();
            transfer.write_function(|data| {
                res_body.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.header_function(|header| {
                raw_headers.push(String::from_utf8_lossy(header).into_owned());
                true
            }).unwrap();
            transfer.perform().unwrap();
        }
        handle.response_code()
    };
    let mut builder = Response::builder();
    for line in raw_headers.into_iter() {
        let (name, value) = match line.find(':') {
            Some(i) => (&line[..i], line[i+1..].trim()),
            None => (line[..].trim(), "")
        };
        builder.header(name, value);
    }
    builder
        .status(code.unwrap() as u16)
        .body(res_body)
}
