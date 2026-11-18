use compiler::api::SmolAPI;

pub struct CLI {}

impl CLI {
    pub fn new() -> Self {
        Self {}
    }
}

impl SmolAPI for CLI {
    fn is_hidden(&self, widget_index: u16) -> bool {
        println!("[API CALL] : is_hidden");
        true
    }

    fn show(&mut self, widget_index: u16) {
        println!("[API CALL] : is_hidden");
    }

    fn get_text(&self, widget_index: u16) -> Option<&str> {
        println!("[API CALL] : is_hidden");
        Some("")
    }

    fn set_text(&mut self, widget_index: u16, text: String) {
        println!("[API CALL] : is_hidden");
    }

    fn set_active_card(&mut self, card_index: u16) {
        println!("[API CALL] : is_hidden");
    }

    fn get_word_def(&self, dict_name: &str, word: &str) -> Option<String> {
        println!("[API CALL] : is_hidden");
        Some("".to_string())
    }

    fn error(&mut self, message: String, pos: core::range::Range<usize>) {
        println!("[API CALL] : is_hidden");
    }

    fn get_handle(&self, widget_name: &str) -> u16 {
        todo!()
    }
}
