use std::io::BufReader;
use std::collections::BTreeMap;

use md5;
use url::form_urlencoded;
use xml;
use xml::writer::events::XmlEvent as XmlEventWriter;
use xml::reader::XmlEvent as XmlEventReader;


/// 详见: 接口规则 > 安全规范
pub fn calc_sign(params: &BTreeMap<String, String>, api_key: &String) -> String {
    // 如果参数的值为空不参与签名；
    let keys = params
        .iter()
        .filter(|pair| {
            pair.0.ne("key") && pair.0.ne("sign") && !pair.1.is_empty()
        })
        .map(|pair| {pair.0.to_string()})
        .collect::<Vec<String>>();

    // 参数名ASCII码从小到大排序（字典序, BTreeMap 默认就是有序的）
    let mut encoder = form_urlencoded::Serializer::new(String::new());
    for key in keys {
        encoder.append_pair(&key, &params[&key]);
    }

    encoder.append_pair("key", api_key);
    let encoded = encoder.finish();

    // 生成 MD5 字符串
    let mut context = md5::Context::new();
    context.consume(encoded.as_bytes());
    let mut digest = String::with_capacity(32);
    for x in &context.compute()[..] {
        digest.push_str(&format!("{:02X}", x));
    }
    digest
}

/// 将`xml`数据解析成`BTreeMap`
pub fn from_xml(data: &Vec<u8>) -> BTreeMap<String, String> {
    let mut params = BTreeMap::new();
    let reader = xml::reader::EventReader::new(data.as_slice());
    let mut tag: String = "".to_string();
    for event in reader {
        match event {
            Ok(XmlEventReader::StartElement{name, ..}) => {
                tag = name.local_name;
            }
            Ok(XmlEventReader::CData(value)) => {
                params.insert(tag.clone(), value);
            }
            Err(e) => {
                println!("Parse xml error: {:?}", e);
                break;
            }
            _ => {}
        }
    }
    params
}

/// 使用`BTreeMap`生成`xml`数据
pub fn to_xml(params: &BTreeMap<String, String>) -> Vec<u8> {
    let mut target: Vec<u8> = Vec::new();
    {
        let mut writer = xml::writer::EmitterConfig::new()
            .write_document_declaration(false)
            .create_writer(&mut target);
        let _ = writer.write::<XmlEventWriter>(XmlEventWriter::start_element("xml").into());
        for (key, value) in params {
            let _ = writer.write::<XmlEventWriter>(XmlEventWriter::start_element(key.as_ref()).into());
            let _ = writer.write::<XmlEventWriter>(XmlEventWriter::characters(value.as_ref()).into());
            let _ = writer.write::<XmlEventWriter>(XmlEventWriter::end_element().into());
        }
        let _ = writer.write::<XmlEventWriter>(XmlEventWriter::end_element().into());
    }
    target
}
