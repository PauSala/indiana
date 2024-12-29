pub fn print_table(headers: Vec<&str>, rows: Vec<Vec<String>>) {
    let mut max = max_non_last_value(&headers, &rows);

    let margin = 2;
    max += margin;

    // Headers
    for header in &headers {
        print!("{:<width$}", header, width = max);
    }
    println!();

    // Rows
    for row in rows {
        for cell in row {
            print!("{:<width$}", cell, width = max);
        }
        println!();
    }
}

fn max_non_last_value(headers: &[&str], rows: &[Vec<String>]) -> usize {
    let mut max = headers[0..headers.len() - 1]
        .iter()
        .max_by_key(|a| a.len())
        .unwrap_or(&"")
        .len();

    rows.iter().for_each(|r| {
        let local_max = r[0..r.len() - 1].iter().max_by_key(|a| a.len());
        if let Some(max_str) = local_max {
            if max_str.len() > max {
                max = max_str.len();
            }
        }
    });
    max
}
