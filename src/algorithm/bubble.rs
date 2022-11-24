// pub fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
//     for i in 0..list.len() {
//         for x in 0..list.len() - 1 - i {
//             if list[x] > list[x + 1] {
//                 list.swap(x, x + 1);
//             }
//         }
//     }
//     list
// }
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubbl_int() {
        let mut list = vec![1, 34, 50, 200, 34, 51, 25, 100, 65];
        bubble_sort(&mut list);
        println!("{:?}  ", list);
        assert_eq!(list, vec![1, 25, 34, 34, 50, 51, 65, 100, 200])
    }
    #[test]
    fn test_bubbl_char() {
        let mut list = vec!['D', 'e', 'A', 'C', 'a', 'W'];
        bubble_sort(&mut list);
        println!("{:?}  ", list);
        assert_eq!(list, vec!['A', 'C', 'D', 'W', 'a', 'e'])
    }
}
