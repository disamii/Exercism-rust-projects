use std::collections::HashMap;

pub fn raindrops(n: u32) -> String {
    let sound: HashMap<u32, String> = HashMap::from([
        (3, "Pling".to_string()),
        (5, "Plang".to_string()),
        (7, "Plong".to_string()),
    ]);
    let mut raindrops = String::new();
    if n % 3 == 0 {
        raindrops.push_str(&sound.get(&3).unwrap().clone())
    }
    if n % 5 == 0 {
        raindrops.push_str(&sound.get(&5).unwrap().clone())
    }
    if n % 7 == 0 {
        raindrops.push_str(&sound.get(&7).unwrap().clone())
    }
    if raindrops.is_empty() {
        raindrops.push_str(&n.to_string());
    }
    raindrops
}
