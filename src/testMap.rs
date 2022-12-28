pub mod MapTEST {
    use std::collections::HashMap;
    use std::collections;
    use::std;
    pub fn algoTestMap(nums : Vec<i32>) -> i32 {
        let mut maxCount = 0;
        let mut prevMaxItem = 0;
        let mut map = HashMap::new();

        for item in nums {
            let count = map.entry(item).or_insert(0);
            *count += 1;
            if *count > maxCount
            {
                maxCount = *count;
                prevMaxItem = item;
            }
        }
        prevMaxItem
    }

    pub fn SortSalaryJobs ( mut salary : Vec<i32>) -> f64 {
        salary.sort();
        let mut sum : i32  = salary.iter().sum();
        let sumF = f64::from(sum - salary[0] - salary[salary.len() - 1]);
        let MidF = f64::from(salary.len() as i32 - 2);
        let midleSalary = sumF / MidF;
        midleSalary
    }

    pub fn num_tilings(n: i32) -> i32 {
        let mut len = n + 1;
        let mut dp : Vec<i32> = vec![0; len as usize];
        dp[0] = 1;
        for i in 1..len {
            dp[i as usize] = i as i32 * dp[i as usize - 1];           
        }
        dp[len as usize - 1]
    }

}