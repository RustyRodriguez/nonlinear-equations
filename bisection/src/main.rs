fn bisection<F>(f: F, mut a: f64, mut b: f64, n: usize) -> Result<f64, &'static str> 
where F: Fn(f64) -> f64,
{
    if f(a) * f(b) >= 0.0 {
        return Err("No sign change");
    }

    let mut c = a;
    
    for i in 0..n {

        c = a + (b - a) / 2.0;

        let f_c = f(c);

        println!("f_c after {} iterations is {:.6}", i, f_c);

        if f_c.abs() < 1e-6 {
            return Ok(c);
        }

        if f(a) * f_c < 0.0 {
            b = c;
        } else {
            a = c;
        }
    }

    Ok(c)
}
    
fn main() {
    let f = |x: f64| x.powi(2) + x - 4.0;

    match bisection(f, 0.0, 4.0, 10) {
        Ok(root) => {
            println!("Root found is {:.5}", root);
        }
        Err(message) => {
            println!("Error: {}", message);
        }
    }
}
