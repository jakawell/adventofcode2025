fn main() {
    println!("Hello, world!");
}

pub fn check_id_halfsies(id: &str) -> bool {
    // odd lengths are automatically valid since they can't be split in two
    if id.len() % 2 != 0 {
        return true;
    }

    let half_length = id.len() / 2;
    let half1 = &id[0..half_length];
    let half2 = &id[half_length..];

    half1 != half2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_121() {
        assert!(check_id_halfsies("121") == true);
    }

    #[test]
    fn valid_1221() {
        assert!(check_id_halfsies("1221") == true);
    }

    #[test]
    fn invalid_55() {
        assert!(check_id_halfsies("55") == false);
    }

    #[test]
    fn invalid_6464() {
        assert!(check_id_halfsies("6464") == false);
    }

    #[test]
    fn invalid_123123() {
        assert!(check_id_halfsies("123123") == false);
    }
}
