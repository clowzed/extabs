pub trait Expand {
    fn expandtabs(&self, n: u8) -> String;
}

impl Expand for String {
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
                _ => {
                    position += 1;
                    result.push(current_char);
                }
            };
        });
        result
    }
}
