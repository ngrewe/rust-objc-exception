extern crate gcc;
use std::process::Command;

fn main() {
    let mut cfg = gcc::Config::new();
    let result = Command::new("gnustep-config")
                   .arg("--objc-flags")
                   .output()
                   .and_then(|o| String::from_utf8(o.stdout)
                             .map_err(|e| ::std::io::Error::new(::std::io::ErrorKind::InvalidData, e)))
                    .unwrap_or(String::from("-fobjc-exceptions"));
    let mut chars : Vec<char> = Vec::new();
    let mut in_quote = false;
    let mut literal_next = false;
    for c in result.chars() {
      if literal_next {
          chars.push(c);
          literal_next = false;
          continue;
      }
      if c == '"' {
        in_quote = !in_quote;
        continue;
      }
      if c == '\\' {
         literal_next = true;
         continue;
      }
      if !in_quote && c.is_whitespace() {
        if chars.len() > 0 {
            let flag = chars.iter().cloned().collect::<String>();
            cfg.flag(&flag);
            chars.clear();
        }
      } else {
          chars.push(c);
      }
    }
    cfg.file("extern/exception.m");
    cfg.compile("libexception.a");
}
