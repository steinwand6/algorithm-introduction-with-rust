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
////////////////
// BUILD-MAX-HEAP(A)
//	A.heap-size = A.length
//	for i = A.length downto 1
//		MAX-HEAPIFY(A, i)
////////////////
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
