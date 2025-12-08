use proconio::input;

#[derive(Clone)]
struct Input {
    n: usize,
    t: usize,
    m: usize,
    k: usize,
    l: isize,
    xy: Vec<(isize, isize)>,
    v: Vec<(isize, isize)>,
}

impl Input {
    fn parse_input() -> Self {
        input! {
            n: usize,
            t: usize,
            m: usize,
            k: usize,
            l: isize,
            xyv: [(isize, isize, isize, isize); n],
        }
        let mut xy: Vec<(isize, isize)> = Vec::new();
        let mut v: Vec<(isize, isize)> = Vec::new();
        for &(x, y, vx, vy) in xyv.iter() {
            xy.push((x, y));
            v.push((vx, vy));
        }

        Self { n, t, m, k, l, xy, v }
    }
}

struct Solver {
    input: Input,
    ans: Vec<(usize, usize, usize)>,
    cost: f64,
}

impl Solver {
    fn new(input: &Input) -> Self {
        let ans: Vec<(usize, usize, usize)> = Vec::new();
        let cost: f64 = 0.0;

        Self { input: input.clone(), ans, cost }
    }

    fn diff(&self, xy1: (isize, isize), xy2: (isize, isize)) -> f64 {
        let l = self.input.l as usize;

        let (x1, y1) = xy1;
        let (x2, y2) = xy2;
        let x_diff = x1.abs_diff(x2);
        let y_diff = y1.abs_diff(y2);
        let x_diff = x_diff.min(l-x_diff);
        let y_diff = y_diff.min(l-y_diff);

        ((x_diff*x_diff + y_diff*y_diff) as f64).sqrt().round()
    }

    fn solve(&mut self) {
        let n = self.input.n;
        let t = self.input.t;
        let m = self.input.m;
        let k = self.input.k;
        let l = self.input.l;
        let xy = self.input.xy.clone();
        let v = self.input.v.clone();

        for i in 0..n {
            let group = i / k;
            let group_i = group * k;
            if i == group_i { continue; }
            self.ans.push((0, group_i, i));
            
            self.cost += self.diff(xy[i], xy[group_i]);
        }
    }

    fn ans(&self) {
        for &(t, i, j) in self.ans.iter() {
            println!("{} {} {}", t, i, j);
        }
    }

    fn score(&self) -> usize {
        let l = self.input.l;
        let n = self.input.n;
        let m = self.input.m;

        let numerator = l as usize * (n - m);
        let denominator = self.cost + 1.0;
        let ratio = numerator as f64 / denominator;
        let score = 10f64.powi(6) * ratio.log2();

        score.round() as usize
    }

    fn result(&self) {
        eprintln!("{{ \"score\": {} }}", self.score());
    }
}

fn main() {
    let input = Input::parse_input();
    let mut solver = Solver::new(&input);

    solver.solve();

    solver.ans();
    solver.result();
}
