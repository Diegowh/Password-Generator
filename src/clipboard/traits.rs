pub trait ClipboardManager {
    fn copy_to_clipboard(&self, text: &str);
}