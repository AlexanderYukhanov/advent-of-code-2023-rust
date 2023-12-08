fn game(t: i64, d: i64) -> i64 {
    // (t - dt) * dt > d => dt^2 - dt * t + d < 0
    // dt1, dt2 = t/2 +/- sqrt(t^2/4 - d)
    let ft = t as f64;
    let fd = d as f64;
    let dt1 = ft / 2.0_f64 - ((ft * ft) / 4.0_f64 - fd).sqrt();
    let dt2 = ft / 2.0_f64 + ((ft * ft) / 4.0_f64 - fd).sqrt();
    return (dt2.ceil() - dt1.floor()) as i64 - 1;
}

fn main() {
    let mut result = 1;
    let games = [(45, 295), (98, 1734), (83, 1278), (73, 1210)];
    for (t, d) in games {
        result *= game(t, d);
    }
    println!("Part 1: {}", result);
    println!("Part 2: {}", game(45988373, 295173412781210))
}
