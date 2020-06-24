/**
 * @Author : Fred Fu
 * @Time: 2020/6/24 11:27 
 */

pub fn fibonacci(n: i32) -> i32 {
    if n < 1 {
        return 0;
    } else if n == 1 || n == 2 {
        return 1;
    } else {
        let mut pre1 = 1;
        let mut pre2 = 1;
        let mut index = 3;
        let mut re = 1;
        while index <= n {
            re = pre1 + pre2;
            pre2 = pre1;
            pre1 = re;
            index += 1;
        }
        re
    }
}
