#[allow(dead_code)]
mod plot;
use plot::visualize_surface_plotly;
const P: f64 = 1.0;
const L: f64 = 1.0;
const T: f64 = 1.0;
const B: f64 = 0.2 * P;

fn f(x: f64, t: f64) -> f64 {
    let tmp = B * (x + t) + 2.0;
    tmp * tmp
}

fn alpha(x: f64) -> f64 {
    // u(x,0)
    x / (2.0 + x * B)
}

fn beta(x: f64) -> f64 {
    //u_t(x,0)
    let tmp = B * x + 2.0;
    -1.0 / (tmp * tmp)
}

fn gamma_0(t: f64) -> f64 {
    //u(0,t)
    0.0
}

fn gamma_1(t: f64) -> f64 {
    1.0 / (B * (t + 1.0) + 2.0)
}

fn alpha_xx(x: f64) -> f64 {
    let tmp = 2.0 + B * x;
    -4.0 * B * B / (tmp * tmp * tmp)
}

fn solve(M: usize, N: usize) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    let h = L / M as f64;
    let tau = T / N as f64;
    let x: Vec<f64> = (0..=M).map(|i| i as f64 * h).collect();
    let t: Vec<f64> = (0..=N).map(|j| j as f64 * tau).collect();
    let mut u = vec![vec![0.0; N + 1]; M + 1];

    for i in 0..=M {
        u[i][0] = alpha(x[i]);
    }

    for i in 1..M {
        // alpha()
        u[i][1] =
            alpha(x[i]) + tau * beta(x[i]) + 0.5 * tau.powi(2) * (alpha_xx(x[i]) + f(x[i], 0.0));
    }

    for j in 0..=N {
        u[0][j] = gamma_0(t[j]);
        u[M][j] = gamma_1(t[j]);
    }

    let s = (tau / h).powi(2);

    for j in 1..N {
        for i in 1..M {
            u[i][j + 1] = s * (u[i + 1][j] + u[i - 1][j]) + 2.0 * (1.0 - s) * u[i][j] - u[i][j - 1]
                + tau.powi(2) * f(x[i], t[j]);
        }
    }

    (u, x, t)
}

fn main() {
    let (u, x, t) = solve(10, 10);

    print!("      t\\x |");
    for i in 0..=10 {
        print!("{:8.3}", x[i]);
    }
    println!("---------+{}", "-".repeat(11 * 8));

    for j in 0..=10 {
        print!("t = {:.3} |", t[j]);
        for i in 0..=10 {
            print!("{:8.5}", u[i][j]); // u[i][j] – значение в узле (x_i, t_j)
        }
        println!();
    }
    visualize_surface_plotly(&u, &x, &t);
}
