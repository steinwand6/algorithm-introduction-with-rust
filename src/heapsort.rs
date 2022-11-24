////////////////
// MAX-HEAPIFY(A,i)
//	l = LEFT(i)
//	r = RIGHT(i)
//	if l <= A.heap-size && A[l] > A[i]
//		largest = l
//	else
//		largest = i
//	if r <= A.heap-size && A[r] > A[largest]
//		largest = r
//	if largest != i
//		swap(A[i], A[largest])
//		MAX-HEAPIFY(A,largest)
////////////////
fn max_heapify(a: &mut Vec<i64>, i: usize) {
    let l = left(a, i);
    let r = right(a, i);
    let mut largest = i;
    if let Some(l) = l {
        if a[l] > a[i] {
            largest = l;
        }
    }
    if let Some(r) = r {
        if a[r] > a[largest] {
            largest = r;
        }
    }
    if largest != i {
        a.swap(i, largest);
        max_heapify(a, largest);
    }
}
fn left(a: &Vec<i64>, i: usize) -> Option<usize> {
    if a.len() > (i * 2) + 1 {
        Some(i * 2 + 1)
    } else {
        None
    }
}
fn right(a: &Vec<i64>, i: usize) -> Option<usize> {
    if a.len() > (i * 2) + 2 {
        Some(i * 2 + 2)
    } else {
        None
    }
}
////////////////
// BUILD-MAX-HEAP(A)
//	A.heap-size = A.length
//	for i = A.length downto 1
//		MAX-HEAPIFY(A, i)
////////////////
fn build_max_heap(a: &mut Vec<i64>) {
    todo!()
}
////////////////
// HEAPSORT(A)
//	BUILD-MAX-HEAP(A)
//	for i = A.length downto 2
//		swap(A[1], A[i])
//		A.heap-size = A.heap-size - 1
//		MAX-HEAPFY(A, 1)
////////////////
pub fn heapsort(a: &Vec<i64>) -> Vec<i64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(
            heapsort(&vec![2, 4, 5, 7, 1, 3, 6]),
            vec![1, 2, 3, 4, 5, 6, 7]
        )
    }
}
