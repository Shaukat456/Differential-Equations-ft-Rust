fn dy_dt(y: f64, k: f64) -> f64 {
    k * y
}

fn euler_method(y0: f64, h: f64, n: u32, k: f64) -> f64 {
    let mut y = y0;

    for _ in 0..n {
        y = y + h * dy_dt(y, k);
    }

    y
}

fn main() {
    let y0 = 1.0;
    let h = 0.01;
    let n = 100;
    let k = -0.5;

    println!("The approximate solution is {}", euler_method(y0, h, n, k));
}
