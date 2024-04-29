use bindings::exports::fermyon::hmac::{sign, verify};
use data_encoding::HEXUPPER;
use ring::hmac;
use wit_bindgen::rt::vec::Vec;
mod bindings;

struct Component;

impl sign::Guest for Component {
    fn sign(data: Vec<u8>, keyvalue: Vec<u8>) -> Result<Vec<u8>, sign::Error> {
        let key = hmac::Key::new(hmac::HMAC_SHA256, &keyvalue);
        let signature = hmac::sign(&key, &data);
        let tag = HEXUPPER.encode(signature.as_ref());
        Ok(tag.into_bytes())
    }
}

impl verify::Guest for Component {
    fn verify(data: Vec<u8>, keyvalue: Vec<u8>, tag: Vec<u8>) -> bool {
        let key = hmac::Key::new(hmac::HMAC_SHA256, &keyvalue);
        let Ok(tag) = HEXUPPER.decode(tag.as_slice()) else {
            return false;
        };
        match hmac::verify(&key, data.as_slice(), tag.as_slice()) {
            Ok(_) => true,
            _ => false,
        }
    }
}
bindings::export!(Component with_types_in bindings);
