use compiler::api::SmolAPI;

pub struct CLI {}

impl CLI {
    pub fn new() -> Self {
        Self {}
    }
}

impl SmolAPI for CLI {
    fn is_hidden(&self, widget_index: u16) -> bool {
        todo!()
    }

    fn show(&mut self, widget_index: u16) {
        todo!()
    }

    fn get_text(&self, widget_index: u16) -> Option<&str> {
        todo!()
    }

    fn set_text(&mut self, widget_index: u16, text: &str) {
        todo!()
    }

    fn set_active_card(&mut self, card_index: u16) {
        todo!()
    }

    fn get_word_def(&self, dict_name: &str, word: &str) -> Option<&str> {
        todo!()
    }

    fn error(&mut self, message: String, pos: core::ops::Range<usize>) {
        todo!()
    }
}
