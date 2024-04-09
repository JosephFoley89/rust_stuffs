use std::collections::HashMap;
use rand::Rng;

pub fn unordered_list_i32() -> [i32; 25] {
	let mut list: [i32; 25] = [25; 25];

	for x in 0..25 {
		list[x] = rand::thread_rng().gen_range(1..=100);
	}

	return list;
}

pub fn fibonacci_hash_u128() -> HashMap<u32, u128> {
	let mut map: HashMap<u32, u128> = HashMap::new();
    map.insert(0, 0);
    map.insert(1, 1);
    map.insert(2, 1);
    map.insert(3, 2);
    map.insert(4, 3);
    map.insert(5, 5);
    map.insert(6, 8);
    map.insert(7, 13);
    map.insert(8, 21);
    map.insert(9, 34);

    return map;
}