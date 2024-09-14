use std::collections::HashMap;

pub fn make_huffmap(feeder: &str) -> HashMap<char, i32>{

    let mut huff_map = HashMap::new();

    for ch in feeder.chars(){
        match huff_map.get(&ch){
            None => { huff_map.insert(ch, 1); },
            Some(T) =>{ huff_map.insert(ch, huff_map.get(&ch).unwrap() + 1); }
        }
    }

    huff_map




}