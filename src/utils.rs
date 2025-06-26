use unicode_width::UnicodeWidthChar;

pub fn visual_to_logical(x: usize, s: &str) -> usize {
    let mut visual_col = 0;

    for (logical_pos, ch) in s.chars().enumerate() {
        let char_width = ch.width().unwrap_or(1);

        // 你  好  呀
        // 0   2  3
        // 0 + 2
        // 2 + 2
        // 4 + 2 > 4
        if visual_col + char_width > x {
            return logical_pos;
        }

        visual_col += char_width;
    }

    s.chars().count()
}
