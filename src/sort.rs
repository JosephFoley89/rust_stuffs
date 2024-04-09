pub fn bubble_sort(mut list: [i32; 25]) -> [i32; 25] {
    let range = list.len() - 1;

    for x in 0..range {
        for y in 0..range - x {
            if list[y] > list[y+1] {
                let temp = list[y + 1];
                list[y + 1] = list[y];
                list[y] = temp;
            }
        }
    }

    return list;
}

pub fn bubble_sort_reverse(mut list: [i32; 25]) -> [i32; 25] {
    let range = list.len() - 1;

    for x in 0..range {
        for y in 0..range - x {
            if list[y] < list[y+1] {
                let temp = list[y+1];
                list[y+1] = list[y];
                list[y] = temp;
            }
        }
    }

    return list;
}