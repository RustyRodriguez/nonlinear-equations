fn newton_raphson_method<F, DF>(f: F, df: DF, x_0: f64, n: u32) -> Result<f64, &'static str>
where
    F: Fn(f64) -> f64,
    DF: Fn(f64) -> f64
{
    let mut x = x_0;

    for _ in 1..n {
        let f_x = f(x);
        let df_x = df(x);

        if df_x == 0.0 {
            return Err("Derivative is 0");
        }

        x = x - (f_x / df_x);
    }

    Ok(x)
}

fn main() {
    let f = |x: f64| x.powi(2) - 2.0;
    let df = |x: f64| 2.0 * x;

    match newton_raphson_method(f, df, 1.0, 100) {
        Ok(root) => {
            println!("The root is {:.5}", root)
        }
        Err(message) => {
            print!("Error Message: {}", message)
        }
    }
}
