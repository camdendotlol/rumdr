use std::collections::HashMap;
use yansi::{Color, Style};

pub struct Config {
  pub filename: String,
  pub theme: Theme
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Missing markdown file")
    };

    let theme_name = match args.next() {
      Some(arg) => arg,
      None => "default".to_string()
    };

    let theme = set_theme(theme_name);

    Ok (Config { filename, theme })
  }
}

fn set_theme(theme_name: String) -> Theme {
    let themes = populate_themes();

    if let Some(t) = Some(themes[&theme_name]) {
      return t
    }

    // Use default theme if argument is not supplied
    themes["default"]
  }

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

// pub fn get_theme(name: &str) -> Theme {
//   let themes = populate_themes();  

//   let theme = match name {
//     "bob" => themes["bob"],
//     _ => themes["default"]
//   };

//   theme
// }