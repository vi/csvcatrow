use std::ffi::{OsStr};

use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    if std::env::args_os().nth(1) == None {
        eprintln!("Note: operating on zero input files. Use --help to show the help message");
    }
    if std::env::args_os().nth(1).as_deref() == Some(OsStr::new("--help")) {
        println!("Usage: csvcatrow file1.csv file2.csv ... fileN.csv > output.csv");
        println!("Concatenate rows from many small csv files into a single table, using column names as keys.");
        println!("Number and order of columns may be different in the input files.");
        std::process::exit(1);
    }

    let mut columns_global : IndexSet<Box<[u8]>> = IndexSet::with_capacity(16);
    let mut args = std::env::args_os();
    args.next();
    for arg in args {
        if arg == "-" {
            eprintln!("Reading from stdin is not supported, as we need to scan the files twice");
            std::process::exit(1);
        }

        let mut f = csv::ReaderBuilder::default().from_path(arg)?;
        let h = f.byte_headers()?;

        for field in h {
            let fi = field.to_vec().into_boxed_slice();
            columns_global.insert(fi);
        }
    }
    
    let so = std::io::stdout();
    let so = so.lock();
    let mut w = csv::WriterBuilder::default().from_writer(so);

    for c in &columns_global {
        w.write_field(c)?;
    }
    w.write_byte_record(&csv::ByteRecord::new())?;

    let mut args = std::env::args_os();
    args.next();
    for arg in args {
        let mut f = csv::ReaderBuilder::default().from_path(&arg)?;
        let h = f.byte_headers()?;

        let mut columns_of_this_file = rustc_hash::FxHashMap::with_capacity_and_hasher(columns_global.len(), Default::default());

        for (n,field) in h.into_iter().enumerate() {
            let fi = field.to_vec().into_boxed_slice();
            if columns_of_this_file.contains_key(&fi) {
                eprintln!(
                    "Warning: dulplicate column `{}` name in file `{:?}`.",
                    String::from_utf8_lossy(&*fi),
                    arg,
                );
            }
            columns_of_this_file.insert(fi, n);
        }

        for row in f.byte_records() {
            let row = row?;
            for c in &columns_global {
                if let Some(idx) = columns_of_this_file.get(c) {
                    if let Some(d) = row.get(*idx) {
                        w.write_field(d)?;
                    } else {
                        w.write_field(b"")?;
                    }
                } else {
                    w.write_field(b"")?;
                }
               
            }
            w.write_byte_record(&csv::ByteRecord::new())?;
        }
    }


    Ok(())
}
