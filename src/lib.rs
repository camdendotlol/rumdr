pub mod config;
mod parse;

pub fn run(text: String, theme: config::Theme) {
  let lines = text.split("\n");

  let mut code_block_found = false;

  for line in lines {
    if line.len() >= 3 && &line[..3] == "```" {

      if code_block_found {
        code_block_found = false
      } else {
        code_block_found = true
      }

    } else if code_block_found {
      println!("{}", theme.code.paint(line))
    } else if line.trim() == "" {
      println!("")
    } else {
      parse::header::handle_headers(line, theme);
      parse::bullet::handle_bullet(line);
      parse::block_quote::handle_block_quote(line);
    }
  }
}
