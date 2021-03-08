pub mod coloring {
    pub enum Colors{
        Red,
        Green,
        Blue
    }

    pub fn color_text(color: Colors, text: &str) -> String {
        let mut start = "";
        let mut end = "\x1b[0m";
        match color {
            Colors::Red => {
                start = "\x1b[1;31m";
            },
            Colors::Green => {
                start = "\x1b[1;32m";
            },
            Colors::Blue => {
                start = "\x1b[1;34m";
            },
            _ => {
                println!("Color not found:(");
                end = "";
            }
        };
        format!("{}{}{}", start, text, end)
    }
}