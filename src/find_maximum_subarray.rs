//FIND-MAX-CROSSING-SUBARRAY(A, low, mid, high)
//	left-sum = -infinity
//	sum = 0
//	for i = mid downto low
//		sum = sum + A[i]
//		if sum > left-sum
//			left-sum = sum
//			max-left = i
//	rigth-sum = -infinity
//	sum = 0
//	for j = mid + 1 to high
//		sum = sum + A[j]
//		if sum > right-sum
//			right-sum = sum
//			max-right = j
//	return (max-left, max-right, left-sum + right-sum)
// ↑線形時間
//
// FIND-MAXIMUM-SUBARRAY(A, low, high)
//	if high == low
//		return (low, high, A[low])
//	mid = (low + high) / 2
//	(left-low, left-high, left-sum) = FIND-MAXIMUM-SUBARRAY(A, low, mid)
//	(left-low, right-high, right-sum) = FIND-MAXIMUM-SUBARRAY(A, mid+1, high)
//	(cross-low, cross-high, cross-sum) = FIND-MAX-CROSSING-SUBARRAY(A, low, mid, high)
//	if left-sum >= right-sum && left-sum >= crossing-sum
//		return (left-low, left-high, left-sum)
//	else if right-sum >= left-sum && right-sum >= crossing-sum
//		return (right-low, right-high, right-sum)
//	return (cross-low, cross-high, cross-sum)

fn find_max_crossing_subarray(
    a: &Vec<i64>,
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, i64) {
    todo!()
}

fn find_maximum_subarray(a: &Vec<i64>, low: usize, high: usize) -> (usize, usize, i64) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        assert_eq!(
            find_maximum_subarray(
                &vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7],
                0,
                15
            ),
            (7, 10, 43)
        )
    }
}
