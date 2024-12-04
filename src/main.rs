// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

fn main() {
    println!("Little Timmy is nice >> {}", is_nice(10, 1));
}

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    // Create the enum variants `Nice` and `Naughty`
    // Variant `Nice` is a tuple struct that holds the number of good deeds
}

pub struct Kid {
    // Add a field `name` of type `String`
    // and `niceness` field of type `Niceness`
    // Make all fields public
}

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
    let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

    let ratio = good_deeds / (good_deeds + bad_deeds);
    if ratio >= 0.75 {
        return true;
    } else {
        return false;
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