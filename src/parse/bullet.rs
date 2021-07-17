pub fn handle_bullet(line: &str) {
  if line.len() < 2 {
    return {}
  }

  let opening = &line[..2];

  match opening {
    "* " | "- " => println!("• {}", &line[2..]),
    _ => {}
  }
}