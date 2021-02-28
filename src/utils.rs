pub mod ib_message {
    use std::convert::TryInto;
    use std::str;
    use rust_decimal::prelude::*;
    pub trait IBMessage {
        fn to_ib_message(&self) -> Result<Vec<u8>, std::num::TryFromIntError>;
    }

    impl IBMessage for &str {
        fn to_ib_message(&self) -> Result<Vec<u8>, std::num::TryFromIntError> {
            let msg_len: u32 = match self.len().try_into() {
                Ok(val) => val,
                Err(err) => return Err(err),
            };
            let len_bytes = msg_len.to_be_bytes();
            let mut res = Vec::with_capacity(self.len() + 4);
            res.extend_from_slice(&len_bytes);
            res.extend_from_slice(&self.as_bytes());
            Ok(res)
        }
    }

    pub fn iter_ib_message(msg: &[u8]) -> std::str::Split<'_, &str> {
        let string_view = str::from_utf8(msg).expect("No valid UTF-8");
        string_view.split("\0")
    }

    
    pub fn decode_enum<T>(stream: &mut std::str::Split<'_, &str>) -> T
    where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
            T::from_str(stream.next().unwrap()).unwrap()
    }

    pub fn decode<T>(stream: &mut std::str::Split<'_, &str>) -> Option<T> 
    where
    T: std::str::FromStr + Sized,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let str_val = stream.next().unwrap();
        println!("{}", str_val);
        match str_val {
            "" | "1.7976931348623157E308" => return None,
            _ => Some(T::from_str(str_val).unwrap())
        }
    }

    pub trait Encodable {
        fn encode(&self) -> String;
    }

    pub trait Nums {}
    impl Nums for f64 {}
    impl Nums for i32 {}
    impl Nums for i64 {}
    impl Nums for Decimal {}
    impl Nums for usize {}

    //encoding for numeric types
    impl<T> Encodable for T
    where
        T: Nums + ToString
    {
        fn encode(&self) -> String {
            self.to_string() +"\0"
        }
    }

    impl Encodable for bool {
        fn encode(&self) -> String {
            if *self {"1\0".to_string()} else {"0\0".to_string()}
        }
    }

    impl Encodable for String
    {
        fn encode(&self) -> String {
            self.to_string() + "\0"
        }
    }

    impl<T: Encodable> Encodable for Option<T> {
        fn encode(&self) -> String {
            match self {
                Some(val) => val.encode(),
                None => "\0".to_string()
            }
        }
    }

    impl Encodable for Vec<(String,String)> {
        fn encode(&self) -> String {
            let mut code = String::new();
            for tv in self {
                code.push_str(&tv.0);
                code.push_str("=");
                code.push_str(&tv.1);
                code.push_str(";");
            }
            code.push_str("\0");
            code
        }
    }

    pub fn push_enc<T: Encodable>(str: &mut String, val: T) {
        str.push_str(&val.encode());
    }
}

pub mod ib_stream {
    use super::ib_message::IBMessage;
    use std::convert::TryInto;
    use std::error::Error;
    use tokio::io::AsyncWriteExt;
    use tokio::io::AsyncReadExt;
    use tokio::net::tcp::OwnedReadHalf;
    use tokio::net::tcp::OwnedWriteHalf;
    pub type AsyncResult<T> = Result<T, Box<dyn Error>>;

    pub struct IBReader {
        tcp: OwnedReadHalf,
        headbuf: [u8;4]
    }

    pub struct IBWriter {
        tcp: OwnedWriteHalf,
    }

    impl IBReader {
        pub fn new(tcp: OwnedReadHalf) -> IBReader {
            IBReader {
                headbuf: [0; 4],
                tcp,
            }
        }
        pub async fn read(&mut self) -> AsyncResult<Vec<u8>> {
            let bytes = self.tcp.read(&mut self.headbuf).await?;
            let msg_size = u32::from_be_bytes(self.headbuf);
            let mut msg = vec![0; msg_size.try_into().unwrap()];
            let bytes = self.tcp.read(&mut msg).await?;
            Ok(msg)
        }
    }
    impl IBWriter {
        pub fn new(tcp: OwnedWriteHalf) -> IBWriter {
            IBWriter {
                tcp
            }
        }

        pub async fn write_raw(&mut self, msg: &[u8]) -> AsyncResult<()> {
            self.tcp.write_all(msg).await?;
            Ok(())
        }

        pub async fn write(&mut self, msg: &str) -> AsyncResult<()> {
            self.tcp.write_all(&msg.to_ib_message().unwrap()).await?;
            Ok(())
        }
    }

}
