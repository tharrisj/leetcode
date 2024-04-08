struct Solution;

impl Solution{
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = students;
        let mut sandwiches = sandwiches;
        while students.len() > 0 && students.contains(&sandwiches[0]) {
            students.remove(students.iter().position(|&x| x == sandwiches[0]).unwrap());
            sandwiches.remove(0);
        }
        students.len() as i32
    }
}

fn main() {
    let students = vec![1,1,0,0];
    let sandwiches = vec![0,1,0,1];
    let ret = Solution::count_students(students, sandwiches);
    assert_eq!(ret, 0);

    println!("All tests passed!");
}