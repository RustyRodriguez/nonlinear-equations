fn secant<F>(
    f: F, 
    x_0: f64, 
    x_1: f64,
    tolerance: f64, 
    max_iterations: u16
) -> Result<f64, &'static str>
where F: Fn(f64) -> f64
{
    let mut x_n = x_1;
    let mut x_n_minus_one = x_0;

    for _ in 0..max_iterations {
        let f_n = f(x_n);
        let f_n_minus_one = f(x_n_minus_one);

        if f_n.abs() < tolerance {
            return Ok(x_n);
        }

        let temp = x_n;

        if f_n == f_n_minus_one {
            return Err("no!!!");
        }

        x_n = x_n - f_n * ((x_n - x_n_minus_one) / (f_n - f_n_minus_one));
        x_n_minus_one = temp;
    }

    Ok(x_n)
}

fn main() {
    let f = |x: f64| x.powi(2) - 2.0;

    match secant(f, 1.0, 2.0, 0.0001, 100) {
        Ok(root) => {
            println!("Root = {:.5}", root);
        }
        Err(message) => {
            println!("Error: {}", message);
        }
    }
}