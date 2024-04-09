pub fn binary_search(list: [i32; 100], search: i32, start: usize, end: usize) -> i32 {
    let mid: usize = ((start + end) / 2).try_into().unwrap();

    if search == list[mid] {
        return mid.try_into().unwrap();
    }

    if start >= end {
        return -1;
    }

    if search < list[mid] {
        binary_search(list, search, start, mid - 1)
    } else {
        binary_search(list, search, mid + 1, end)
    }
}