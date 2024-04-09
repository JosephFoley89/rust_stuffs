pub fn bubble_sort(mut list: [i32; 25], reverse: bool) -> [i32; 25] {
    let range = list.len() - 1;

    for x in 0..range {
        for y in 0..range - x {
            if reverse {
                if list[y] < list[y+1] {
                    let temp = list[y + 1];
                    list[y + 1] = list[y];
                    list[y] = temp;
                }
            } else {
                if list[y] > list[y+1] {
                    let temp = list[y + 1];
                    list[y + 1] = list[y];
                    list[y] = temp;
                }
            }
        }
    }

    return list;
}