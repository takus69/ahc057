use proconio::input;
use ac_library::Dsu;

#[derive(Clone)]
struct Input {
    n: usize,
    t: usize,
    m: usize,
    k: usize,
    l: isize,
    xy: Vec<(f64, f64)>,
    v: Vec<(f64, f64)>,
}

impl Input {
    fn parse_input() -> Self {
        input! {
            n: usize,
            t: usize,
            m: usize,
            k: usize,
            l: isize,
            xyv: [(f64, f64, f64, f64); n],
        }
        let mut xy: Vec<(f64, f64)> = Vec::new();
        let mut v: Vec<(f64, f64)> = Vec::new();
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

    fn diff(&self, xy1: (f64, f64), xy2: (f64, f64)) -> f64 {
        let l = self.input.l as f64;

        let (x1, y1) = xy1;
        let (x2, y2) = xy2;
        let x_diff = (x1 - x2).abs();
        let y_diff = (y1 - y2).abs();
        let x_diff = x_diff.min(l-x_diff);
        let y_diff = y_diff.min(l-y_diff);

        (x_diff*x_diff + y_diff*y_diff).sqrt().round()
    }

    fn add_v(&self, m1: usize, v1: (f64, f64), m2: usize, v2: (f64, f64)) -> (f64, f64) {
        let m1 = m1 as f64;
        let m2 = m2 as f64;
        let vx = (m1*v1.0 + m2*v2.0) / (m1+m2); 
        let vy = (m1*v1.1 + m2*v2.1) / (m1+m2); 

        (vx, vy)
    }

    fn r#move(&self, xy: (f64, f64), v: (f64, f64), t: usize) -> (f64, f64) {
        let l = self.input.l as f64;
        let (mut x, mut y) = xy;
        let (vx, vy) = v;
        let t = t as f64;
        x += vx * t;
        y += vy * t;
        if x < 0.0 { x += l; } else if x >= l { x -= l; }
        if y < 0.0 { y += l; } else if y >= l { y -= l; }

        (x, y)
    }

    fn join(&self, i: usize, j: usize, dsu: &mut Dsu, now_v: &Vec<(f64, f64)>) -> (usize, (f64, f64)) {
        let i_size = dsu.size(i);
        let j_size = dsu.size(j);
        let i_leader = dsu.leader(i);
        let j_leader = dsu.leader(j);
        let i_v = now_v[i_leader];
        let j_v = now_v[j_leader];
        let new_leader = dsu.merge(i, j);

        (new_leader, self.add_v(i_size, i_v, j_size, j_v))
    }

    fn solve(&mut self) {
        let (ans, cost) = self.greedy();

        self.ans = ans;
        self.cost = cost;
    }

    fn greedy(&self) -> (Vec<(usize, usize, usize)>, f64) {
        let n = self.input.n;
        let t = self.input.t;
        let m = self.input.m;
        let k = self.input.k;
        let l = self.input.l;
        let xy = self.input.xy.clone();
        let v = self.input.v.clone();

        // グループの決定と重心速度の算出
        let mut groups: Vec<Vec<usize>> = vec![Vec::new(); m];
        let mut g_v: Vec<(f64, f64)> = vec![(0.0, 0.0); m];
        let mut g_m: Vec<usize> = vec![0; m];
        let mut g_xy: Vec<(f64, f64)> = vec![(0.0, 0.0); m];
        for i in 0..n {
            let group = i / k;
            groups[group].push(i);
            g_v[group] = self.add_v(g_m[group], g_v[group], 1, v[i]);
            g_m[group] += 1;
            g_xy[group].0 += xy[i].0 / k as f64;
            g_xy[group].1 += xy[i].1 / k as f64;
        }

        let mut ans: Vec<(usize, usize, usize)> = Vec::new();
        let mut cost: f64 = 0.0;
        // グループの重心との最短距離を算出
        let mut min_diff: Vec<f64> = vec![l as f64/10.0; n];
        for ti in 0..t {
            for gi in 0..m {
                let g_xy_ti = self.r#move(g_xy[gi], g_v[gi], ti);

                for &i in groups[gi].iter() {
                    let xy_ti = self.r#move(xy[i], v[i], ti);
                    let diff = self.diff(g_xy_ti, xy_ti);
                    min_diff[i] = min_diff[i].min(diff);
                    // min_diff[i] = min_diff[i].max(l as f64/20.0);
                }
            }
        }

        // 最短距離より近いグループの要素があったら結合(グループの初期は一番小さい要素)
        let mut now_xy = xy.clone();
        let mut now_v = v.clone();
        let mut dsu = Dsu::new(n);
        for ti in 0..t {
            // グループごとに現時点の距離を確認
            for gi in 0..m {
                for &i in groups[gi].iter() {
                    if dsu.size(i) == k { continue; }
                    for &j in groups[gi].iter() {
                        if i == j || dsu.same(i, j) { continue; }
                        let diff = self.diff(now_xy[i], now_xy[j]);
                        if min_diff[i] >= diff && min_diff[j] >= diff {
                            // 結合
                            let (new_leader, new_v) = self.join(i, j, &mut dsu, &now_v);
                            ans.push((ti, i, j));
                            cost += diff;
                            now_v[new_leader] = new_v;
                        }
                    }
                }
            }

            // 1回進める
            for i in 0..n {
                let leader = dsu.leader(i);
                now_xy[i] = self.r#move(now_xy[i], now_v[leader], 1);
            }
        }

        // 結合できてないものは最後に結合
        eprintln!("not join: {}", n-m-ans.len());
        for gi in 0..m {
            for &i in groups[gi].iter() {
                if dsu.size(i) == k { continue; }
                for &j in groups[gi].iter() {
                    if i == j || dsu.same(i, j) { continue; }
                    let diff = self.diff(now_xy[i], now_xy[j]);
                    let (new_leader, new_v) = self.join(i, j, &mut dsu, &now_v);
                    ans.push((t-1, i, j));
                    cost += diff;
                    now_v[new_leader] = new_v;
                }
            }
        }

        eprintln!("ans: {}, n-m: {}", ans.len(), n-m);

        (ans, cost)
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
