
use std::{fs::File, io::{self, Read}};

use base64::prelude::*;

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // println!("Read {} bytes from input", buffer.len());

    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&buffer),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(&buffer),
    };

    println!("{}", encoded);
    Ok(encoded)
}
pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    // 输出的内容可能包含换行符或空格，进行修剪
    // 以确保解码时不会出错
    // trim -> 返回一个去除字符串开头和结尾所有空白字符的新字符串切片, 包含换行符、空格、回车、制表等
    let buffer = buffer.trim();

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(&buffer)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(&buffer)?,
    };

    // TODO decode data might not be string(but for this case, we assume it is)
    let decoded_str = String::from_utf8(decoded)?;

    println!("{}", decoded_str);
    Ok(decoded_str)
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)

    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() -> anyhow::Result<()> {
        let input = "fixtures/tmp.b64.revert";
        let encoded = process_encode(input, Base64Format::Standard)?;
        assert_eq!(encoded, "SGVsbG8gV29ybGQh");
        Ok(())
    }

    #[test]
    fn test_process_decode() -> anyhow::Result<()> {
        let input = "fixtures/tmp.b64";
        let decoded = process_decode(input, Base64Format::Standard)?;
        assert_eq!(decoded, "Hello World!");
        Ok(())
    }
}
