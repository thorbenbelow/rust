#[derive(PartialEq, Debug)]
pub struct StrSplit<'haystack, D> {
  remainder: Option<&'haystack str>,
  delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
  pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
    Self {
      remainder: Some(haystack),
      delimiter,
    }
  }
}

pub trait Delimiter {
  fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
  D: Delimiter,
{
  type Item = &'haystack str;
  fn next(&mut self) -> Option<Self::Item> {
    let remainder = self.remainder.as_mut()?;
    if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
      let until_delim = &remainder[..delim_start];
      *remainder = &remainder[delim_end..];
      Some(until_delim)
    } else {
      self.remainder.take()
    }
  }
}

impl Delimiter for &str {
  fn find_next(&self, s: &str) -> Option<(usize, usize)> {
    let next_delim = s.find(self)?;
    Some((next_delim, next_delim + self.len()))
  }
}

impl Delimiter for char {
  fn find_next(&self, s: &str) -> Option<(usize, usize)> {
    s.char_indices()
      .find(|(_, c)| c == self)
      .map(|(start, _)| (start, start + self.len_utf8()))
  }
}

fn until_char(s: &str, c: char) -> &str {
  StrSplit::new(s, c)
    .next()
    .expect("StrSplit always gives at least one result")
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
  }

  #[test]
  fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
  }

  #[test]
  fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
  }
}
