pub trait PasswordGenerator {
    fn generate(&self, secret: &str, service: &str) -> String;
}