pub trait Expand {
    fn expandtabs(&self, n: u8) -> String;
}

impl Expand for String {
    /// Return a new string where all tab
    /// characters are replaced by one or more spaces,
    /// depending on the current column and the given tab size.
    /// Tab positions occur every tabsize characters
    /// To expand the string, the current column is set to zero
    /// and the string is examined character by character.
    /// If the character is a tab (\t), one or more space characters are inserted
    /// in the result until the current column is equal to the next tab position.
    /// (The tab character itself is not copied.)
    /// If the character is a newline (\n) or return (\r), it is copied and the current
    /// column is reset to zero.
    /// Any other character is copied unchanged and the current column is incremented
    /// by one regardless of how the character is represented when printed.
    ///
    /// Usage
    /// ```
    /// let s = String::from("Hello\tWorld!");
    /// let expanded = s.expandtabs(4);
    /// println!("{}", expanded);
    /// 
    /// ```
    ///
    fn expandtabs(&self, n: u8) -> String {
        let mut result = String::new();

        let mut position = 0;

        self.chars().into_iter().for_each(|current_char| {
            match current_char {
                '\t' => {
                    position = 0;
                    result += &" ".repeat((n - position % n).into());
                }
                '\n' => {
                    position = 0;
                    result.push(current_char);
                }
                '\r' => {
                    position = 0;
                    result.push(current_char);
                }
                _ => {
                    position += 1;
                    result.push(current_char);
                }
            };
        });
        result
    }
}
