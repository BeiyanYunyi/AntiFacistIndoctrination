use sha2::{Digest, Sha512};

pub fn sha512(string: &String) -> String {
  let mut hasher = Sha512::new();
  hasher.update(string);
  let hash = hasher.finalize();
  let mut buf = [0u8; 512];
  return String::from(base16ct::lower::encode_str(&hash, &mut buf).unwrap());
}
