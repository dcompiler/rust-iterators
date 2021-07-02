
#[cfg(test)]
mod tests{

    use std::collections::HashMap;
    use std::iter::Iterator;

    #[derive(Copy,Clone)]
    pub struct PPUC {
        ppuc: i64,
        lease: i64,
        old_lease: i64,
        ref_id: i64,
    }

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

    #[test]
    fn test_fold() {
        let total = [(1,2),(2,3)].iter().fold(0,|acc,(k,v)| acc+v);
        assert_eq!(total, 5);
    }

    #[test]
    fn hashmap_iters() {
        let mut hm : HashMap<i32, (i32,i32)> = HashMap::new();
        hm.insert(1, (2,3));
        hm.insert(4, (5,6));
        let inner2 : HashMap<i32,i32> = hm.iter().fold(HashMap::new(), |mut acc, (&k,&v)| {acc.insert(v.0,v.1); acc});
        let mut inner = HashMap::new();
        for (_, &v) in hm.iter() {
            inner.insert(v.0, v.1);
        }
        assert_eq!(inner.len(), 2);
        let prod : i32 = inner.iter().fold(1, |acc, (&k,&v)| acc*v);
        assert_eq!(prod, 18);
        let prod : i32 = inner2.iter().fold(1, |acc, (&k,&v)| acc*v);
        assert_eq!(prod, 18);
    }

}
