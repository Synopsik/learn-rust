use num::Complex;


pub fn test_initial_loops() {
    // square_loop(0.99);
    // square_add_loop(0.20);
    // complex_square_add_loop(Complex::new(0.24, 2.10));
}


fn square_loop(mut x: f64) {
    loop {
        if x > 0.0 && x < 100.0 {
            println!("{}", x);
            x = x * x;
        } else {
            return
        }
    }
}


fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        println!("{}", x);

        x = x * x + c;

        if x == f64::INFINITY { return }
    }
}


fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}


pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}