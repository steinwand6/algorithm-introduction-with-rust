// p <= q < r (それぞれは配列の要素 を指す添字)
// 部分列A[p..q]とa[q+1..r]はソート済み
// MERGE(A, p, q, r)
//     n1 = q - p + 1
//     n2 = r - q
//     let L[1..n1] and R[1..n2] be new arrays
//     for i = 1 to n1
//         L[i] = A[p + i - 1]
//     for j = 1 to n2
//         R[j] = A[q + j]
//     L[n1 + 1] = infinity
//     L[n2 + 1] = infinity
//     i = 1
//     j = 1
//     for k = p to r
//         if L[i] <= R[j]
//             A[k] = L[i]
//             i = i + 1
//         else
//             A[k] = R[j]
//             j = j + 1

pub fn merge_sort(array: &Vec<i64>, p: usize, q: usize, r: usize) -> Vec<i64> {
    let n1 = q - p;
    let n2 = r - q;
    let mut array_l: Vec<i64> = vec![];
    let mut array_r: Vec<i64> = vec![];

    for i in 0..n1 {
        array_l.insert(i, array[i]);
    }
    for j in 0..n2 {
        array_r.insert(j, array[q + j]);
    }
    array_l.insert(n1, i64::MAX);
    array_r.insert(n2, i64::MAX);

    let mut result = Vec::clone(array);

    let mut i = 0;
    let mut j = 0;
    println!("{:?}", array_l);
    println!("{:?}", array_r);
    for k in p..r {
        if array_l[i] <= array_r[j] {
            result[k] = array_l[i];
            i += 1;
        } else {
            result[k] = array_r[j];
            j += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(
            merge_sort(&vec![2, 4, 5, 7, 1, 3, 6], 0, 4, 7),
            vec![1, 2, 3, 4, 5, 6, 7]
        )
    }
}
