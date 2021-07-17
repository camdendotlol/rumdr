mod parse;

pub fn run(text: String) {
  let lines = text.split("\n");

  for line in lines {
    if line.trim() == "" {
      println!("")
    } else {
      parse::header::handle_headers(line);
      parse::bullet::handle_bullet(line);
      parse::block_quote::handle_block_quote(line);
    }

  }
}
