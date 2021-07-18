use std::{error::Error, fs};

pub mod config;
mod parse;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
  let text = fs::read_to_string(config.filename)?;

  let lines = text.split("\n");

  let mut code_block_found = false;

  Ok(for line in lines {
    if line.len() >= 3 && &line[..3] == "```" {

      if code_block_found {
        code_block_found = false
      } else {
        code_block_found = true
      }

    } else if code_block_found {
      println!("{}", config.theme.code.paint(line))
    } else if line.trim() == "" {
      println!("")
    } else {
      parse::header::handle_headers(line, config.theme);
      parse::bullet::handle_bullet(line);
      parse::block_quote::handle_block_quote(line);
    }
  })
}
