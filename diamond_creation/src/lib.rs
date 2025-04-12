use std::iter;

pub fn get_diamond(c: char) -> Vec<String> {
    // Calculate the size of the diamond (height and width)
    let size = ((c as u8) - b'A') as usize * 2 + 1;

    // Build the top half of the diamond (including the middle row)
    let half: Vec<_> = iter::once(format!("{0:^1$}", 'A', size)) // First row with 'A' centered
        .chain((1..=size / 2).map(|i| {
            format!(
                "{0:^1$}", // Center the inner formatted string to total width 'size'
                format!("{0}{1}{0}", // Construct row like 'B B', 'C   C', etc.
                    (b'A' + i as u8) as char, // Current letter
                    " ".repeat(i * 2 - 1) // Spaces between the letters
                ),
                size // Total width of the row
            )
        }))
        .collect();

    // Create full diamond by adding reversed top half (excluding middle row)
    half.iter()
        .chain(half.iter().rev().skip(1)) // Skip the middle row to avoid duplication
        .cloned() // Clone each String from the iterator
        .collect() // Collect into a Vec<String>
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(get_diamond('A'), vec!["A"]);
    }

    #[test]
    fn test_b() {
        assert_eq!(get_diamond('B'), vec![" A ", "B B", " A "]);
    }

    #[test]
    fn test_c() {
        assert_eq!(
            get_diamond('C'),
            vec!["  A  ", " B B ", "C   C", " B B ", "  A  "]
        );
    }

    #[test]
    fn test_d() {
        assert_eq!(
            get_diamond('D'),
            vec!["   A   ", "  B B  ", " C   C ", "D     D", " C   C ", "  B B  ", "   A   ",]
        );
    }

    #[test]
    fn test_z() {
        assert_eq!(
            get_diamond('Z'),
            vec![
                "                         A                         ",
                "                        B B                        ",
                "                       C   C                       ",
                "                      D     D                      ",
                "                     E       E                     ",
                "                    F         F                    ",
                "                   G           G                   ",
                "                  H             H                  ",
                "                 I               I                 ",
                "                J                 J                ",
                "               K                   K               ",
                "              L                     L              ",
                "             M                       M             ",
                "            N                         N            ",
                "           O                           O           ",
                "          P                             P          ",
                "         Q                               Q         ",
                "        R                                 R        ",
                "       S                                   S       ",
                "      T                                     T      ",
                "     U                                       U     ",
                "    V                                         V    ",
                "   W                                           W   ",
                "  X                                             X  ",
                " Y                                               Y ",
                "Z                                                 Z",
                " Y                                               Y ",
                "  X                                             X  ",
                "   W                                           W   ",
                "    V                                         V    ",
                "     U                                       U     ",
                "      T                                     T      ",
                "       S                                   S       ",
                "        R                                 R        ",
                "         Q                               Q         ",
                "          P                             P          ",
                "           O                           O           ",
                "            N                         N            ",
                "             M                       M             ",
                "              L                     L              ",
                "               K                   K               ",
                "                J                 J                ",
                "                 I               I                 ",
                "                  H             H                  ",
                "                   G           G                   ",
                "                    F         F                    ",
                "                     E       E                     ",
                "                      D     D                      ",
                "                       C   C                       ",
                "                        B B                        ",
                "                         A                         ",
            ]
        );
    }
}