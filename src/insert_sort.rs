// p14
// INSERTION-SORT(A)
//     for j = 2 to A.length
//         key = A[j]
//         i = j - 1
//         while i > 0 and A[i] < key
//             A[i + 1] = A[i]
//             i = i - 1
//         A[i + 1] = key

pub fn insert_sort(array: &Vec<i64>) -> Vec<i64> {
    #[warn(dead_code)]
    let mut array: Vec<i64> = Vec::clone(array);

    for j in 1..array.len() {
        let mut i = j;
        while i > 0 && array[i] < array[i - 1] {
            array.swap(i, i - 1);
            i = i - 1;
        }
    }
    array
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(insert_sort(&vec! {5,2,4,6,1,3}), vec! {1,2,3,4,5,6})
    }
    #[test]
    fn test_1_element() {
        assert_eq!(insert_sort(&vec! {1}), vec! {1})
    }
    #[test]
    fn test_no_element() {
        assert_eq!(insert_sort(&vec! {}), vec! {})
    }
}
