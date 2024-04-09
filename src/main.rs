mod sort;
mod test_data;
mod tabular;

fn main() {
    let test_array: [i32; 25] = test_data::unordered_list_i32();
    let test_ordered_list: [i32; 25] = sort::bubble_sort(test_array, false);
    let test_reveresed_list: [i32; 25] = sort::bubble_sort(test_array, true);
    let mut tabular_list: Vec<tabular::Tabular> = vec![];
    let record_headers: Vec<&str> = vec![" Original", " Sorted", " Reversed"];

    for x in 0..25 {
        tabular_list.push(tabular::Tabular  { 
            original: test_array[x], 
            sorted: test_ordered_list[x], 
            reversed: test_reveresed_list[x] 
        });
    }

    tabular::tabulate(record_headers, tabular_list);
}