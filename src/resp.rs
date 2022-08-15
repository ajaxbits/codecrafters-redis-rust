use std::convert::TryInto;
use std::i32;
use std::str;

enum Resp {
    SimpleString(String),
    BulkString(String),
    Array(String),
    Integer(String),
    Error(String),
}

fn extract_resp_data(data: &[u8]) -> Vec<u8> {
    let resp_range = 1..(data.len() - 2);
    data[resp_range].to_owned()
}

fn handle_string(data: &[u8]) -> String {
    let buf = extract_resp_data(data);
    match str::from_utf8(&buf) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

fn handle_integer(data: &[u8]) -> i32 {
    let buf = extract_resp_data(data);
    let LENGTH: usize = buf.len();
    let array = <&[u8; LENGTH]>::try_from(buf.as_slice());
    let value = i32::from_ne_bytes(if cfg!(target_endian = "big") {
        int_array
    } else {
        int_array.reverse();
        int_array
    });
    value
}

fn parse_resp(resp: Resp) {
    // match resp {
    //     Resp::SimpleString(bytes) => handle_string(bytes),
    //     Resp::Integer(bytes) => handle_integer(bytes),
    //     // Resp::BulkString(bytes) => handle_bulk_string(bytes),
    //     // Resp::Array(bytes) => handle_array(bytes),
    //     // Resp::Error(bytes) => todo!(),
    // }
    // Ok()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extract_resp_data() {
        let pong = b"+PONG\r\n";
        let wow = b"+wow\r\n";
        let numbers = b"+1231231231\r\n";
        assert_eq!(extract_resp_data(pong), b"PONG");
        assert_eq!(extract_resp_data(wow), b"wow");
        assert_eq!(extract_resp_data(numbers), b"1231231231");
    }

    #[test]
    fn test_handle_string() {
        let pong = b"+PONG\r\n";
        let wow = b"+wow\r\n";
        assert_eq!(handle_string(pong), "PONG".to_string());
        assert_eq!(handle_string(wow), "wow".to_string());
    }

    #[test]
    fn test_handle_integer() {
        assert_eq!(handle_integer(b"+1283197\r\n"), 1283197);
    }
}
