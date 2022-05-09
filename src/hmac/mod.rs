
use ::hmac::{Hmac, Mac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

pub fn calculate_hmac<'a, 'b>(key: &'b str, data: &'a str) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

#[cfg(test)]
mod tests {
    use crate::hmac::calculate_hmac;
    #[test]
    fn it_calculates_hmac() {
        let result = calculate_hmac("hsm", "test");
        assert_eq!(
            result,
            "9ca7ff355a7531b395c8ccb16d8c8ccf03319b53c11ee76ed3c51962690aaef5"
        );
    }
}
