pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub struct Kid {
    pub name: String,
    pub good_deeds: u32,
    pub bad_deeds: u32,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        Kid {
            name: name,
            good_deeds: good_deeds,
            bad_deeds: bad_deeds,
        }
    }

    pub fn is_nice(self: &Self) -> bool {
        let good_deeds = self.good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = self.bad_deeds as f32 * BAD_WEIGHT;
        
        let ratio = good_deeds / (good_deeds + bad_deeds);
        if ratio >= 0.75 {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nice() {
        let timmy = Kid::new(String::from("timmy"), 10 , 1);
        let result_true = timmy.is_nice();
        assert_eq!(result_true, true);

        let timmy = Kid::new(String::from("timmy"), 4 , 10);
        let result_false = timmy.is_nice();
        assert_eq!(result_false, false);

        let timmy = Kid::new(String::from("timmy"), 0 , 0);
        let result_default = timmy.is_nice();
        assert_eq!(result_default, false);
    }
}
