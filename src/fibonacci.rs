use std::collections::HashMap;

pub fn get_fibonacci(mut map: HashMap<u32, u128>, key: u32) -> HashMap<u32, u128> {
	if map.contains_key(&key) {
		return map;
	} else {
		let start: u32 = map.len().try_into().unwrap();
		for x in start..key+1 {
			map.insert(x, map[&(x - 1)] + map[&(x - 2)]);
		}
	}
	
	return map;
}