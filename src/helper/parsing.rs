pub struct Columns<'a>(usize, usize, &'a [&'a str]);

impl<'a> Iterator for Columns<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == self.1 {
            return None;
        }

        let out = Some(
            self.2
                .iter()
                .map(|l| l.chars().nth(self.0).unwrap())
                .collect::<String>(),
        );
        self.0 += 1;
        out
    }
}

pub trait IntoColumns<'a> {
    fn into_columns(self) -> Columns<'a>;
}

impl<'a> IntoColumns<'a> for &'a [&'a str] {
    fn into_columns(self) -> Columns<'a> {
        if self.is_empty() {
            return Columns(0, 0, &[]);
        }

        let min_len: usize = self.iter().map(|s| s.len()).min().unwrap();

        Columns(0, min_len, self)
    }
}

mod tests {
    use super::IntoColumns;

    #[test]
    pub fn into_columns() {
        assert_eq!(
            ["a b c", "d e f"]
                .as_slice()
                .into_columns()
                .collect::<Vec<String>>(),
            vec!["ad", "  ", "be", "  ", "cf"]
        );
    }

    #[test]
    pub fn filter_columns() {
        assert_eq!(
            ["a b c", "d e f"]
                .as_slice()
                .into_columns()
                .filter(|c| !c.trim().is_empty())
                .collect::<Vec<String>>(),
            vec!["ad", "be", "cf"]
        );
    }
}
