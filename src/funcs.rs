const VARIABEL: i32 = 5;

pub fn testare(z: i32) -> i32 {
    5 + z + VARIABEL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid_return_from_testare() -> Result<(), String> {
        let result = testare(2);
        if result == 12 {
            Ok(())
        } else {
            Err(format!("Expected 12, got {}", result))
        }
    }

    #[test]
    fn check_valid_return_from_testare_2() -> Result<(), String> {
        let result = testare(5);
        if result == 15 {
            Ok(())
        } else {
            Err(format!("Expected 15, got {}", result))
        }
    }
}
