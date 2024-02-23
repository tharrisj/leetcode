struct Solution;

impl Solution{
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adjacency: Vec<Vec<[i32;2]>> = vec![vec![];n as usize];
        
        for flight in flights {
            adjacency[flight[1] as usize].push([flight[0],flight[2]])
        }

        let result = Self::diijkstra(adjacency,dst as usize,src as usize,k as usize)[src as usize];
        if result == i32::MAX { -1 } else { -result }
    }

    fn diijkstra(graph: Vec<Vec<[i32;2]>>, start:usize, end: usize,limit: usize) -> Vec<i32> {
        let mut result : Vec<i32> = vec![i32::MAX;graph.len()];
        
        let mut queue: std::collections::BinaryHeap<(i32,usize,usize)> = std::collections::BinaryHeap::new();
        queue.push((0,start,0));

        while let Some((path_cost,node,stops)) = queue.pop(){
            if path_cost >= result[node] || stops >= limit + 2{ continue; }
            result[node] = path_cost;

            if node == end { break; }
            
            for neighbor in &graph[node] {
                if result[neighbor[0] as usize] != i32::MAX { continue; }
                queue.push((path_cost-neighbor[1],neighbor[0] as usize,stops +1));
            }

        }
        result
    }
}

fn main() {
    let n = 4;
    let flights = vec![vec![0,1,100], vec![1,2,100], vec![2,0,100], vec![1,3,600], vec![2,3,200]];
    let src = 0;
    let dst = 3;
    let k = 1;
    let ret = Solution::find_cheapest_price(n, flights, src, dst, k);
    assert_eq![ret, 700];

    println!("Tests passed!");
}