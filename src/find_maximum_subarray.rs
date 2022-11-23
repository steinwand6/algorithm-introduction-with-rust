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
    let mut max_left = mid;
    let mut left_sum = i64::MIN;
    let mut sum = 0;
    for i in (low..=mid).rev() {
        sum += a[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut max_right = mid;
    let mut right_sum = i64::MIN;
    sum = 0;
    for j in (mid + 1)..=high {
        sum = sum + a[j];
        if sum > right_sum {
            right_sum = sum;
            max_right = j;
        }
    }
    (max_left, max_right, left_sum + right_sum)
}

fn find_maximum_subarray(a: &Vec<i64>, low: usize, high: usize) -> (usize, usize, i64) {
    if high == low {
        return (low, high, a[low]);
    }
    let mid = (low + high) / 2;
    let (left_low, left_high, left_sum) = find_maximum_subarray(a, low, mid);
    let (right_low, right_high, right_sum) = find_maximum_subarray(a, mid + 1, high);
    let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(a, low, mid, high);
    if left_sum >= right_sum && left_sum >= cross_sum {
        return (left_low, left_high, left_sum);
    } else if right_sum >= left_sum && right_sum >= cross_sum {
        return (right_low, right_high, right_sum);
    }
    (cross_low, cross_high, cross_sum)
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
