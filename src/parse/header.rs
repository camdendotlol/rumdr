use yansi::Paint;

pub fn handle_headers(line: &str) {
  if line.len() < 4 {
    return {}
  }

  let opening = line[..4].chars();

  let mut is_header = false;
  let mut header_level: u8 = 0;
  for char in opening {
    if char == '#' {
      header_level += 1
    } else {
      if char == ' ' {
        is_header = true;
        break
      } else {
        break
      }
    }
  }

  if is_header {
    println!("Level {} header: {}", header_level, Paint::green(line))
  }
}