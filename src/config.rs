use std::collections::HashMap;
use yansi::{Color, Style};

#[derive(Clone, Copy)]
pub struct Theme {
  pub header: Style,
  pub code: Style
}

impl Theme {
  fn new(header: Style, code: Style) -> Theme {
    Theme { header, code }
  }
}

fn populate_themes() -> HashMap<String, Theme> {
  let mut themes = HashMap::new();

  // These will eventually be read from a config file
  themes.insert("default".to_string(), Theme::new(
    Color::Green.style(),
    Color::White.style().dimmed()
  ));

  themes.insert("bob".to_string(), Theme::new(
    Color::RGB(214, 160, 126).style(),
    Color::White.style().dimmed()
  ));

  themes
}

pub fn get_theme(name: &str) -> Theme {
  let themes = populate_themes();  

  let theme = match name {
    "bob" => themes["bob"],
    _ => themes["default"]
  };

  theme
}