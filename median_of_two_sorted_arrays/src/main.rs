fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let middle = (nums1.len() + nums2.len())/2;
    let average: bool = (nums1.len() + nums2.len()) % 2 == 1;

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut prev = 0;
    let median = loop {
        let cur = match (nums1.get(i), nums2.get(j)) {
                    (Some(&n1), Some(&n2)) => n1.min(n2),
                    (Some(&n1), None) => n1,
                    (None, Some(&n2)) => n2,
                    (None, None) => 0,
        };

        if i+j >= middle {
            match average {
                true => { break cur as f64; },
                false => { break (cur as f64 + prev as f64)/2.0; },
            }
        }

        match (nums1.get(i), nums2.get(j)) {
            (Some(&n1), Some(&n2)) => { if n1 > n2 { j += 1; } else { i+= 1; } },
            (Some(_), None) => i += 1,
            (None, Some(_)) => j += 1,
            (None, None) => (),
        }

        prev = cur;
    };

    median
}

fn main() {
    let m1 = vec!(1,3);
    let m2 = vec!(2);
    let ret = find_median_sorted_arrays(m1, m2);
    println!("Median is: {}", ret);

    let m1 = vec!(1,2);
    let m2 = vec!(3,4);
    let ret = find_median_sorted_arrays(m1, m2);
    println!("Median is: {}", ret);

    let m1 = vec!(1, 3, 5, 7, 9);
    let m2 = vec!(0, 2, 4, 6, 8, 10);
    let ret = find_median_sorted_arrays(m1, m2);
    println!("Median is: {}", ret);

    let m1: Vec<i32> = vec!();
    let m2 = vec!(2, 3);
    let ret = find_median_sorted_arrays(m1, m2);
    println!("Median is: {}", ret);
}
