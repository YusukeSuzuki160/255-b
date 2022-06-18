use proconio::input;
fn dist(x_1: isize, x_2: isize, y_1: isize, y_2: isize) -> f64 {
    (((x_1 - x_2).pow(2) + (y_1 - y_2).pow(2)) as f64).sqrt()
}

fn near_light(x: &Vec<isize>, y: &Vec<isize>, light_list: &Vec<usize>) -> Vec<f64> {
    let mut dist_list: Vec<f64> = Vec::new();
    for i in 0..x.len() {
        if light_list.contains(&(i + 1)) {
            dist_list.push(0.0);
        } else {
            let mut min_dist: f64 = std::f64::MAX;
            for j in light_list {
                let dist = dist(x[i], x[*j - 1], y[i], y[*j - 1]);
                if dist < min_dist {
                    min_dist = dist;
                }
            }
            dist_list.push(min_dist);
        }
    }
    dist_list
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        z: [(isize, isize); n],
    }
    let mut x = Vec::new();
    let mut y = Vec::new();
    for i in z {
        let (x_in, y_in) = i;
        x.push(x_in);
        y.push(y_in);
    }
    let near_list = near_light(&x, &y, &a);
    let mut ans = 0.0;
    for i in near_list {
        if ans < i {
            ans = i;
        }
    }
    println!("{}", ans);
}
