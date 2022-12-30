mod fort;

use fort::FortTable;

fn main() {
    let mut t = FortTable::new();
    t.add_header(&["N", "Driver", "Time", "Avg Speed"]);
    t.add_row(&["1", "Ricciardo", "1:25.945", "222.128"]);
    t.add_row(&["2", "Hamilton", "1:26.373", "221.027"]);
    t.add_row(&["3", "Verstappen", "1:26.469", "220.782"]);
    println!("{t}");
}
