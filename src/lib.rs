pub mod count_down {
    use itertools::Itertools;
    use std::fmt;
    use std::sync::Arc;
    use rayon::prelude::*;
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    pub type Int = u32;
    type Cache = HashMap<u64, Result>;

    pub fn populate_cache() -> Cache {
        let mut cache: Cache = HashMap::new();
        const CANDIDATES: [u32; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 25, 50, 75, 100];
        for first in CANDIDATES.iter() {
            for second in CANDIDATES.iter() {
                for op in [Add, Sub, Mul, Div].iter().filter(|op| valid(op, *first, *second)) {
                    cache.insert(key(op, first, second), _make(op, &Val(*first), first, &Val(*second), second));
                }
            }
        }
        return cache;
    }

    lazy_static! {
    static ref CACHE: Cache = populate_cache();
    }

    #[derive(Copy, Clone)]
    pub enum Op {
        Add,
        Sub,
        Mul,
        Div,
    }

    #[derive(Clone)]
    pub enum Expr {
        Val(Int),
        App(Op, Arc<Expr>, Arc<Expr>),
    }

    use Expr::*;
    use Op::*;

    fn valid(op: &Op, x: Int, y: Int) -> bool {
        match op {
            Add => x <= y,
            Sub => x > y,
            Mul => x != 1 && y != 1 && x <= y,
            Div => y > 1 && ((x % y) == 0),
        }
    }

    fn apply(op: &Op, a: Int, b: Int) -> Int {
        match op {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
        }
    }

    fn split<T>(xs: &[T]) -> Vec<(&[T], &[T])> {
        let xsl = xs.len();
        match xsl {
            1 => vec!((&xs[..1], &xs[..1])),
            2 => vec!((&xs[0..1], &xs[1..2])),
            _ => (1..xsl).map(|i| xs.split_at(i)).collect()
        }
    }

    fn sub_bags<T: Clone>(xs: Vec<T>) -> Vec<Vec<T>> {
        (0..xs.len() + 1)
            .flat_map(|i| xs.iter().cloned().permutations(i))
            .collect()
    }

    type Result = (Expr, Int);

    fn _make (op: &Op, l: &Expr, x: &Int, r: &Expr, y: &Int) -> Result {
        (App(*op, Arc::new(l.clone()), Arc::new(r.clone())),
         apply(op, *x, *y))
    }

    fn key(op: &Op, l: &u32, r: &u32) -> u64 {
        return ((*op as u64) << 61) | ((*l as u64) << 30) | (*r as u64);
    }

    fn make (op: &Op, l: &Expr, x: &Int, r: &Expr, y: &Int) -> Result {

        if let Val(lv) = l {
            if let Val(rv) = r {
                let cached = CACHE.get(&key(op, lv, rv)).unwrap().clone();
                return cached;
            }
        }

        return _make(op, l, x, r, y);
    }

    fn combine((l, x): &Result, (r, y): &Result) -> Vec<Result> {
        [Add, Sub, Mul, Div].iter()
            .filter(|op| valid(op, *x, *y))
            .map(|op| make(op, l, x, r, y))
            .collect()
    }

    fn results(ns: &[Int]) -> Vec<Result> {
        match ns {
            [] => vec!(),
            [n] => vec!((Val(*n), *n)),
            _ => _results(ns),
        }
    }

    // NOTE: creating a Vec seems wasteful, why don't we return
    // an iterator? Used to be complex, but this SO answer shows
    // an elegant approach: https://stackoverflow.com/a/58683171/25735

    fn _results(ns: &[Int]) -> Vec<Result> {
        split(ns).iter()
            .flat_map(|(ls, rs)| {
                let lr = results(ls);
                let rr = results(rs);
                let mut vr: Vec<Result> = Vec::new();
                for l in &lr {
                    for r in &rr {
                        vr.append(&mut combine(&l, &r));
                    }
                }
                return vr;
            })
            .collect()
    }

    pub fn solutions(ns: Vec<Int>, n: Int) -> Vec<Expr> {
        sub_bags(ns).par_iter()
            .flat_map(|bag|
                results(&bag).into_iter()
                    .filter(|(_, m)| *m == n)
                    .map(|(e, _)| e)
                    .collect::<Vec<Expr>>()
            )
            .collect()
    }

    // Utilities for displaying expressions
    fn get_str(expr: &Expr) -> String {
        match &*expr {
            Val(v) => v.to_string(),
            App(op, l, r) => format!("({} {:?} {})", get_str(&l), op, get_str(&r)),
        }
    }

    impl fmt::Debug for Op {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Add => write!(f, "+"),
                Sub => write!(f, "-"),
                Mul => write!(f, "*"),
                Div => write!(f, "/"),
            }
        }
    }

    impl fmt::Debug for Expr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", get_str(self))
        }
    }
}