use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut individual_tree=BTreeMap::new();
     for (key,values) in h.iter(){
        for  value in values.iter() {
                        let lower_char = value.to_lowercase().next().unwrap();

            individual_tree.insert (lower_char,*key);
        }
     }
     individual_tree
}
