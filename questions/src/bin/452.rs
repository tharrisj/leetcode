struct Solution;

impl Solution{
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable();
        let mut common_points = (points[0][0], points[0][1]);
        let mut count = 1;

        for i in 1..points.len() {
            if common_points.1 >= points[i][0] {
                common_points = (
                    common_points.0.max(points[i][0]),
                    common_points.1.min(points[i][1])
                );
            } else {
                common_points = (points[i][0], points[i][1]);
                count += 1;
            }
        }

        count
    }
}

fn main() {
    let points = vec![vec![10,16], vec![2,8], vec![1,6], vec![7,12]];
    let ret = Solution::find_min_arrow_shots(points);
    assert_eq!(ret, 2);

    let points = vec![vec![1,2], vec![3,4], vec![5,6], vec![7,8]];
    let ret = Solution::find_min_arrow_shots(points);
    assert_eq!(ret, 4);
    

    println!("All tests passed!");
}