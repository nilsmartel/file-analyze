use walkdir::WalkDir;

fn main() {
    let entrypath = std::env::args().nth(1).unwrap_or(".".to_string());

    let walker = WalkDir::new(entrypath);

    let mut filetypes = std::collections::HashMap::<String, usize>::new();

    for entry in walker {
        if entry.is_err() {
            continue;
        }

        let entry = entry.unwrap();

        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };

        if name.is_empty() || !name.contains(".") || name.starts_with(".") {
            continue;
        }

        let ty = name.split(".").last().unwrap().to_string();

        if ty.is_empty() {
            continue;
        }

        let e = filetypes.entry(ty).or_insert(0);
        *e += 1;
    }

    let mut s = filetypes.iter().collect::<Vec<_>>();
    s.sort_unstable_by_key(|n| n.1);

    for (k, v) in s.into_iter().rev() {
        println!("{k}:  {v}");
    }
}
