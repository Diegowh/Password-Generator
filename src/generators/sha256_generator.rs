use sha2::{Sha256, Digest};
use crate::generators::traits::PasswordGenerator;

pub struct Sha256PasswordGenerator;

impl PasswordGenerator for Sha256PasswordGenerator {
    fn generate(&self, secret: &str, service: &str) -> String {
        let data = format!("{}-{}", secret, service);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)[..16].to_string()
    }
}