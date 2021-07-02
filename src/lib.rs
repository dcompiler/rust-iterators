
#[cfg(test)]
mod tests{

    use std::collections::HashMap;
    use std::iter::Iterator;

    #[test]
    fn test_scan() {
        let a = [1, 2, 3];
        let mut iter2 = a.iter().scan(0, |mut acc, e| {*acc += e; Some(*acc)});
        let prefix_sum : Vec<i32> = iter2.collect();
        assert_eq!(prefix_sum, [1,3,6]);
    }
}
