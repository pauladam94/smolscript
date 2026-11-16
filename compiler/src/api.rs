use alloc::string::String;

pub trait SmolAPI {
    /// Returns the widget index.
    fn get_handle(&self, widget_name: &str) -> u16;

    /// Is widget at `widget_index` hidden ?
    fn is_hidden(&self, widget_index: u16) -> bool;

    /// Unhides widget at `widget_index`.
    fn show(&mut self, widget_index: u16);

    /// Returns the text of the widget at `widget_index` if it is a textual widget.
    fn get_text(&self, widget_index: u16) -> Option<&str>;

    /// Sets the text of widget at `widget_index` to `text` if it is a textual widget.
    fn set_text(&mut self, widget_index: u16, text: alloc::string::String);

    /// Sets the card at `card_index` as active if the card exists.
    fn set_active_card(&mut self, card_index: u16);

    /// Returns the definition of `word` from the dictionary `dict_name`
    /// if the dictionary and the word exist.
    fn get_word_def(&self, dict_name: &str, word: &str) -> Option<alloc::string::String>;

    /// Signal from compiler that an error occured. The error is composed
    /// of a message and a position in the source.
    fn error(&mut self, message: String, pos: core::range::Range<usize>);
}

// go check in wren_module

/*
// card = active card
Wren code

widget.onClick = Fn.new {
  cards[3].setActive()
}

set_active_card(3);

--------------------------------------------------------------------------------


widget.onClick = Fn.new {
  var lang = card["lang"]
  if (lang.get() == "fr/en") {
    lang.set("en/fr")
  } else {
    lang.set("fr/en")
  }
}

if (lang.text == "fr-en") {
  lang.text = "en-fr";
} else {
  lang.text = "fr-en";
}

--------------------------------------------------------------------------------

import "smolphone/dict" for Dict

widget.onClick = Fn.new {
  var dict_name = card["lang"].get().replace("/", "-")
  var word = card["word"].get()
  var result = Dict.getWordDef(dict_name, word)
  if (result == null) {
    result = "word not found"
  }
  card["content"].set(result.toString)
}

let dict_name = car

--------------------------------------------------------------------------------

widget.onClick = Fn.new {
  var text = card["new-item"].get()
  if (card["item1"].isHidden()) {
    card["text1"].set(text)
    card["item1"].show()
  } else if (card["item2"].isHidden()) {
    card["text2"].set(text)
    card["item2"].show()
  }
}



*/
