
#[cfg(test)]
mod tests{

    use std::collections::HashMap;
    use std::iter::Iterator;

    #[test]
    fn test_scan() {
        let a = [1, 2, 3];
        let prefix_sum : Vec<i32> = a.iter().scan(0, |acc, e| {*acc += e; Some(*acc)}).collect();
        assert_eq!(prefix_sum, [1,3,6]);
    }

    #[test]
    fn test_filter() {
        let a = [1, 2, 3];
        let greaterthan1 : Vec<i32> = a.iter().filter(|&&x| x > 1).map(|x| *x).collect();
        assert_eq!(greaterthan1, [2,3]);
    }
}
