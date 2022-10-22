pub fn process_file(filename: String, header_field_index: usize, header_level: usize) -> String {
    let mut reader = csv::Reader::from_path(filename).expect("Failure to read CSV file");
    let headers = reader.headers().expect("Failure getting headers").clone();

    let entries: Vec<_> = reader
        .records()
        .map(|row| {
            let mut result_str = String::new();
            let row = row.expect("Failure parsing string");

            let header = row.get(header_field_index).unwrap_or("Unnamed");
            result_str.push_str(&format!("{} {}\n", "#".repeat(header_level), header));

            for (i, cell) in row.into_iter().enumerate() {
                if i != header_field_index {
                    if !cell.is_empty() {
                        result_str.push_str(&format!(
                            "**{}:** {}\n\n",
                            headers.get(i).unwrap_or("Unknown"),
                            cell
                        ));
                    }
                }
            }

            result_str
        })
        .collect();

    entries.join("")
}
