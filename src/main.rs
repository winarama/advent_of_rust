// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

fn main() {
    println!("Little Timmy is nice >> {}", is_nice(10, 1));
}

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    let gdw = good_deeds as f64 * GOOD_WEIGHT as f64;
    let bdw = bad_deeds as f64 * BAD_WEIGHT as f64;

    let ratio = gdw / (gdw + bdw);
    if ratio >= 0.75 {
        return true
    } else {
        return false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nice() {
        let result_true = super::is_nice(10, 1);
        assert_eq!(result_true, true);

        let result_false = super::is_nice(4, 10);
        assert_eq!(result_false, false);

        let result_default = super::is_nice(0, 0);
        assert_eq!(result_default, false);
    }
}