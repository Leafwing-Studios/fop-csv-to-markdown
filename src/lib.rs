use std::{fs, path::PathBuf};

pub fn process_file(filename: &PathBuf, header_field_index: usize, header_level: usize) -> String {
    let mut reader = csv::Reader::from_path(filename).expect("Failure to read CSV file");
    let headers = reader.headers().expect("Failure getting headers").clone();

    let entries: Vec<_> = reader
        .records()
        .map(|row| {
            let mut result_str = String::new();
            let row = row.expect(&format!("Failure parsing string for file {:?}", filename));

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

pub fn process_dir(filename: PathBuf, header_field_index: usize, header_level: usize) {
    let dir_contents = fs::read_dir(filename).expect("Failure reading directory");

    for path in dir_contents {
        let path = path.expect("Failure reading path");

        if path
            .metadata()
            .expect("Failure reading path metadata")
            .is_file()
        {
            if let Some(extension) = path.path().extension() {
                if extension == "csv" {
                    let mut path = path.path();

                    let header_field_index =
                        if path.file_name().unwrap().to_str().unwrap() == "features.csv" {
                            1
                        } else {
                            header_field_index
                        };
                    let markdown = process_file(&path, header_field_index, header_level);
                    path.set_extension("md");
                    fs::write(path, markdown).expect("Failure writing output file");
                }
            }
        } else {
            process_dir(path.path(), header_field_index, header_level);
        }
    }
}
