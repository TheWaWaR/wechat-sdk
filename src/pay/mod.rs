
mod utils;

use std::collections::BTreeMap;

use http::{Method, Request, Response};
use url::Url;
// use reqwest::{Client, Request, Response, Method, Body};

use ::utils::{Certificate, get_nonce_str, http_post};
use self::utils::{calc_sign, from_xml, to_xml};

const API_BASE_URL: &'static str = "https://api.mch.weixin.qq.com/";

pub enum ResultItem {
    Map(BTreeMap<String, ResultItem>),
    Str(String)
}

pub struct WeChatPayClient {
    // 微信公众号 appid
    appid: String,
    // 商户 key
    api_key: String,
    // 商户号
    mch_id: String,
    // 可选，子商户号，受理模式下必填
    sub_mch_id: Option<String>,
    // 必填，商户证书路径
    mch_cert: String,
    // 必填，商户证书私钥路径
    mch_key: String,

    // other fields
    api_base_url: String,
}


impl WeChatPayClient {
    pub fn new<S: ToString>(
        appid: S,
        api_key: S,
        mch_id: S,
        sub_mch_id: Option<S>,
        mch_cert: S,
        mch_key: S,
        api_base_url: Option<S>,
    ) -> Self {
        let api_base_url = api_base_url
            .map(|s| s.to_string())
            .unwrap_or(API_BASE_URL.to_string());
        WeChatPayClient {
            appid: appid.to_string(),
            api_key: api_key.to_string(),
            mch_id: mch_id.to_string(),
            sub_mch_id: sub_mch_id.map(|s| s.to_string()),
            mch_cert: mch_cert.to_string(),
            mch_key: mch_key.to_string(),
            api_base_url,
        }
    }

    fn request(&self, req: Request<Vec<u8>>) -> BTreeMap<String, ResultItem> {
        let cert = Certificate::Pem{
            cert: self.mch_cert.clone(),
            key: self.mch_key.clone(),
            password: "".into()
        };
        let response = match req.method().clone() {
            Method::POST => {
                http_post(req, Some(cert))
            }
            m @ _ => { panic!(format!("unsupported method: {}", m)) }
        };
        self.handle_result(response.unwrap())
    }

    pub fn post<S: ToString>(
        &self,
        endpoint: S,
        mut data: BTreeMap<String, String>
    ) -> BTreeMap<String, ResultItem> {
        for (key, value) in vec![
            ("mch_key", self.mch_key.clone()),
            ("sub_mch_id", self.sub_mch_id.clone().unwrap_or("".into())),
            ("nonce_str", get_nonce_str()),
        ] {
            data.insert(key.into(), value);
        }
        let sign = calc_sign(&data, &self.api_key);
        data.insert("sign".into(), sign);
        let url = self.get_url(endpoint);
        let req = Request::builder()
            .method(Method::POST)
            .body(to_xml(&data))
            .unwrap();
        self.request(req)
    }

    fn handle_result(
        &self,
        response: Response<Vec<u8>>,
    ) -> BTreeMap<String, ResultItem> {
        let result = BTreeMap::new();
        result
    }

    pub fn get_url<S: ToString>(&self, endpoint: S) -> Url {
        let url_str = format!("{}{}", self.api_base_url, endpoint.to_string());
        Url::parse(&url_str).unwrap()
    }

    pub fn check_signature() {}

    pub fn parse_payment_result() {}
}


pub struct WeChatRedpack<'a> {
    client: &'a WeChatPayClient,
}

impl<'a> WeChatRedpack<'a> {
    pub fn new(client: &'a WeChatPayClient) -> Self {
        WeChatRedpack{ client: client }
    }
}
