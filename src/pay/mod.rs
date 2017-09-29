
use reqwest::Method;

const API_BASE_URL: &'static str = "https://api.mch.weixin.qq.com/";

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
}


impl WeChatPayClient {
    pub fn new<S: ToString>(
        appid: S,
        api_key: S,
        mch_id: S,
        sub_mch_id: Option<S>,
        mch_cert: S,
        mch_key: S,
    ) -> Self {
        WeChatPayClient {
            appid: appid.to_string(),
            api_key: api_key.to_string(),
            mch_id: mch_id.to_string(),
            sub_mch_id: sub_mch_id.map(|s| s.to_string()),
            mch_cert: mch_cert.to_string(),
            mch_key: mch_key.to_string(),
        }
    }

    fn request(method: Method) {}

    fn handle_result() {}

    pub fn get() {}

    pub fn post() {}

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
