// #[cfg(test)]

// mod tests {
  use std::fs::File;
  use std::io::Read;
  use std::path::Path;

  pub fn load_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut s = String::new();
    let _ = File::open(&path).and_then(|mut f| f.read_to_string(&mut s));
    s
  }
// }
