// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.


// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
    fn maybe_icecream(time_of_day: u16) -> Option<u16> {
        // 检查time_of_day是否在有效范围内（0到23）。
        if time_of_day <= 23 {
            // 如果在晚上10点之前，冰箱里还有5块冰淇淋。
            if time_of_day < 22 {
                Some(5)
            } else {
                // 在晚上10点及以后，冰箱里就没有冰淇淋了。
                Some(0)
            }
        } else {
            // 如果time_of_day不在有效范围内，则返回None。
            None
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12);
        match icecreams {
            Some(value) => assert_eq!(value, 5),
            None => panic!("Expected Some, but got None."),
        }
    }
}
