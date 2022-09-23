// Iterative
fn sum_of_squared_digits(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        let m = n % 10;
        n /= 10;
        sum += m * m;
    }
    sum
}

// Functional
/*fn sum_of_squared_digits(n: i32) -> i32 {
    n.to_string().chars().fold(0, |sum: u32, c: char| -> u32 {
        sum + c.to_digit(10).unwrap().pow(2)
    }) as i32
}*/

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        // Detect Cycles with a HashSet
        // Version 1
        /*let mut seen: HashSet<i32> = HashSet::from([n]);

        while sum_of_squared_digits(n) != 1 {
            n = sum_of_squared_digits(n);
            if !seen.insert(n) { return false }
        }
        true*/

        // Version 2
        /*let mut seen: HashSet<i32> = HashSet::new();

        while n != 1 && !seen.contains(&n) {
            seen.insert(n);
            n = sum_of_squared_digits(n);
        }
        return n == 1*/

        // Hardcoding the Only Cycle - {4, 16, 37, 58, 89, 145, 42, 20}
        /*let cycle_members: HashSet<i32> = HashSet::from([4, 16, 37, 58, 89, 145, 42, 20]);

        while n != 1 && !cycle_members.contains(&n) {
            n = sum_of_squared_digits(n);
        }
        return n == 1*/

        // Hardcoding the Only Cycle - 4
        while n > 4 {
            n = sum_of_squared_digits(n);
        }
        return n == 1
    }

    //pub fn is_happy(n: i32) -> bool {
        // Floyd's Cycle-Finding Algorithm
        /*let mut slow_runner = n;
        let mut fast_runner = sum_of_squared_digits(n);

        while fast_runner != 1 && slow_runner != fast_runner {
            slow_runner = sum_of_squared_digits(slow_runner);
            fast_runner = sum_of_squared_digits(sum_of_squared_digits(fast_runner));
        }

        return fast_runner == 1*/
    //}
}
