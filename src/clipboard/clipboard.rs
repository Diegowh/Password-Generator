use crate::clipboard::traits::ClipboardManager;

pub struct ProductionClipboardManager;

impl ClipboardManager for ProductionClipboardManager {
    fn copy_to_clipboard(&self, text: &str) {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                match clipboard.set_text(text) {
                    Ok(_) => {
                        println!("Contraseña copiada al portapapeles");
                    }
                    Err(e) => {
                        eprintln!("Error copiando al portapapeles: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error accediendo al portapapeles: {}", e);
            }
        }
    }
}

pub struct MockClipboardManager;

impl ClipboardManager for MockClipboardManager {
    fn copy_to_clipboard(&self, text: &str) {
        println!("Copiado al portapapeles: {}", text);
    }
}