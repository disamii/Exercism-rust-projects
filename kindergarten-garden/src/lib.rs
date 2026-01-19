pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let pos = students.iter().position(|s| *s == student).unwrap();

    fn plant_from_char(c: char) -> &'static str {
        match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => "",
        }
    }

    let rows: Vec<&str> = diagram.lines().collect();
    let mut result: Vec<&str> = Vec::new();

    for row in &rows {
        let slice = &row[pos * 2 .. pos * 2 + 2];
        for c in slice.chars() {
            result.push(plant_from_char(c));
        }
    }

    result
}
