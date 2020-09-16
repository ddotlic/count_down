pub mod count_down {
    use itertools::Itertools;
    use std::fmt;
    use std::sync::Arc;
    use rayon::prelude::*;
    use lazy_static::lazy_static;

    pub type Int = u32;
    type Cache = Vec<Option<Result>>;

    fn num_index(num: &Int) -> usize {
        let res = match num {
            1..=10 => num - 1,
            15 => 10,
            20 => 11,
            25 => 12,
            50 => 13,
            75 => 14,
            100 => 15,
            _ => panic!("Impossible candidate number")
        };
        res as usize
    }

    fn cache_index(op: &Op, l: &Int, r: &Int) -> usize {
        ((*op as usize) << 8) + (num_index(l) << 4) + num_index(r)
    }

    pub fn populate_cache() -> Cache {
        let mut cache: Cache = vec![None; 1024];
        const CANDIDATES: [Int; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 25, 50, 75, 100];
        for first in CANDIDATES.iter() {
            for second in CANDIDATES.iter() {
                for op in [Add, Sub, Mul, Div].iter().filter(|op| valid(op, *first, *second)) {
                    cache[cache_index(op, first, second)] =
                        Some(_make(op, &Val(*first), first, &Val(*second), second));
                }
            }
        }
        return cache;
    }

    lazy_static! {
    pub static ref CACHE: Cache = populate_cache();
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
        (1..xs.len()).map(|i| xs.split_at(i)).collect()
    }

    fn sub_bags(xs: Vec<Int>, n: Int) -> Vec<Vec<Int>> {
        let start = if xs.iter().any(|i| *i == n) { 1 } else { 2 };
        (start..xs.len() + 1)
            .flat_map(|i| xs.iter().cloned().permutations(i))
            .collect()
    }

    type Result = (Expr, Int);

    fn _make(op: &Op, l: &Expr, x: &Int, r: &Expr, y: &Int) -> Result {
        (App(*op, Arc::new(l.clone()), Arc::new(r.clone())),
         apply(op, *x, *y))
    }

    fn make(op: &Op, l: &Expr, x: &Int, r: &Expr, y: &Int) -> Result {
        if let Val(lv) = l {
            if let Val(rv) = r {
                return CACHE[cache_index(op, lv, rv)].as_ref().unwrap().clone();
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

    fn _combined(split: &(&[Int], &[Int])) -> Vec<Result> {
        let lr = results(split.0);
        let rr = results(split.1);
        let mut vr: Vec<Result> = Vec::new();
        for l in &lr {
            for r in &rr {
                vr.append(&mut combine(&l, &r));
            }
        }
        return vr;
    }

    fn _results(ns: &[Int]) -> Vec<Result> {
        split(ns).iter()
            .flat_map(_combined)
            .collect()
    }

    pub fn solutions(ns: Vec<Int>, n: Int) -> Vec<Result> {
        sub_bags(ns, n).par_iter()
            .flat_map(|bag|
                results(&bag).into_iter()
                    .filter(|(_, m)| *m == n)
                    .collect::<Vec<Result>>()
            )
            .collect()
    }

    fn priority(op: &Op) -> u8 {
        match op {
            Add | Sub => 1,
            Mul | Div => 2,
        }
    }

    static LEFT_P: &str = "(";
    static RIGHT_P: &str = ")";
    static EMPTY_S: &str = "";

    fn get_rpn(expr: &Expr) -> String {
        match &*expr {
            Val(v) => v.to_string(),
            App(op, l, r) => {
                return format!("{} {} {:?}", get_rpn(&l),get_rpn(&r), op);
            }
        }
    }

    fn get_str(expr: &Expr, parent_op: &Op) -> String {
        match &*expr {
            Val(v) => v.to_string(),
            App(op, l, r) => {
                let use_paren = priority(parent_op) > priority(op);
                let (start, end) = if use_paren { (LEFT_P, RIGHT_P) } else { (EMPTY_S, EMPTY_S) };
                return format!("{}{} {:?} {}{}", start, get_str(&l, op), op, get_str(&r, op), end);
            }
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
            write!(f, "{} RPN: {}", get_str(self, &Sub), get_rpn(self))
        }
    }
}