use textwrap::termwidth;

use crate::config::Theme;

pub fn handle_headers(line: &str, theme: Theme) {
  if line.len() < 4 {
    return {}
  }

  let opening = line[..4].chars();

  let mut is_header = false;
  let mut header_level = 0;
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
    let terminal_size = termwidth();

    // Strip the header formatting characters
    let mut string_line = String::from(&line[header_level+1..]);

    for _n in string_line.len()..terminal_size {
      match header_level {
        1 => string_line.push('_'),
        2 => string_line.push('='),
        3 => string_line.push('.'),
        _ => {}
      } 
    }

    let styles = vec![
      // Level 1 headers
      theme.header,
      // Level 2 headers
      theme.header,
      // Level 3 headers
      theme.header
    ];

    println!("{}", styles[header_level-1].paint(string_line));
  }
}