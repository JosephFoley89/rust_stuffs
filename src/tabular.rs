pub struct Tabular {
	pub original: i32,
	pub sorted: i32,
	pub reversed: i32
}

pub fn tabulate(headers: Vec<&str>, records: Vec<Tabular>) {
    println!("+---------------+---------------+---------------+");
    println!("|{}\t|{}\t|{}\t|",  headers[0], headers[1], headers[2]);
    println!("+---------------+---------------+---------------+");

    for x in 0..records.len() {
        println!(
            "|\t{}\t|\t{}\t|\t{}\t|",
            records[x].original,
            records[x].sorted,
            records[x].reversed
        );

        println!("+---------------+---------------+---------------+");
    }   
}