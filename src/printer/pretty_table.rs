pub type Row = [String; 3];

pub fn print_table(headers: Vec<&str>, rows: Vec<Row>) {
    let max_by_col = max_by_column(&headers, &rows);

    let margin = 4;

    // Headers
    for (i, header) in headers.iter().enumerate() {
        print!("{:<width$}", header, width = max_by_col[i] + margin);
    }
    println!();

    // Rows
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            print!("{:<width$}", cell, width = max_by_col[i] + margin);
        }
        println!();
    }
}

fn max_by_column(headers: &[&str], rows: &[Row]) -> Vec<usize> {
    let mut max = vec![0; headers.len()];

    for (i, header) in headers.iter().enumerate() {
        let local_max = rows.iter().map(|r| r[i].len()).max().unwrap_or(0);
        max[i] = std::cmp::max(local_max, header.len());
    }
    max
}
