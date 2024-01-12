#[derive(Debug)]
struct CustomObject {
    name: String,
}

fn fuzzy_search<'a, T>(array: &'a [T], target: &str, tolerance: usize, extract: fn(&T) -> &str) -> Vec<&'a T> {
    array
        .iter()
        .filter(|&item| {
            let item_str = (extract)(item);
            fuzzy_match(item_str, target, tolerance)
        })
        .collect()
}

fn fuzzy_match(s1: &str, s2: &str, tolerance: usize) -> bool {
    let mut s1_iter = s1.chars().fuse();
    let mut s2_iter = s2.chars().fuse();
    let mut errors = 0;

    while let (Some(c1), Some(c2)) = (s1_iter.next(), s2_iter.next()) {
        if c1 != c2 {
            errors += 1;
            if errors > tolerance {
                return false;
            }
        }
    }

    errors <= tolerance
}

fn main() {
    let data = [
        CustomObject { name: "apple".to_string() },
        CustomObject { name: "banana".to_string() },
        CustomObject { name: "cherry".to_string() },
        CustomObject { name: "date".to_string() },
        CustomObject { name: "grape".to_string() },
        CustomObject { name: "kiwi".to_string() },
    ];

    let target = "aple";
    let tolerance = 2;

    let results = fuzzy_search(&data, target, tolerance, |obj| &obj.name);

    println!("Fuzzy search results for '{}': {:#?}", target, results);
}
