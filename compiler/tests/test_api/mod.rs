use compiler::api::SmolAPI;

pub struct TestAPI {}

impl SmolAPI for TestAPI {
    fn is_hidden(&self, _widget_index: u16) -> bool {
        true
    }

    fn show(&mut self, _widget_index: u16) {}

    fn get_text(&self, _widget_index: u16) -> Option<&str> {
        Some("Get Text")
    }

    fn set_text(&mut self, _widget_index: u16, text: &str) {}

    fn set_active_card(&mut self, _card_index: u16) {}

    fn get_word_def(&self, _dict_name: &str, _word: &str) -> Option<&str> {
        Some("Get Text")
    }

    fn error(&mut self, _message: String, _pos: core::range::Range<usize>) {}
}
