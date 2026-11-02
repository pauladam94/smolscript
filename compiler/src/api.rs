trait SmolAPI {
    /// Is widget at `widget_index` hidden ?
    fn is_hidden(&self, widget_index: u16) -> bool;

    /// Unhides widget at `widget_index`.
    fn show(&mut self, widget_index: u16);

    /// Returns the text of the widget at `widget_index` if it is a textual widget.
    fn get_text(&self, widget_index: u16) -> Option<&str>;

    /// Sets the text of widget at `widget_index` to `text` if it is a textual widget.
    fn set_text(&mut self, widget_index: u16, text: &str);

    /// Sets the card at `card_index` as active if the card exists.
    fn set_active_card(&mut self, card_index: u16);

    /// Returns the definition of `word` from the dictionary `dict_name`
    /// if the dictionary and the word exist.
    fn get_word_def(&self, dict_name: &str, word: &str) -> Option<&str>;
}
