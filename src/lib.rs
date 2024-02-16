/* Integer to Roman Logic*/

pub fn int_to_roman(num: u32) -> String {
    let mut num = num;
    let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let symbols = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let mut result = String::new();

    for (value, symbol) in values.iter().zip(symbols.iter()) {
        while num >= *value {
            num -= *value;
            result.push_str(symbol);
        }
    }

    result
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(int_to_roman(1), "I");
        assert_eq!(int_to_roman(2), "II");
        assert_eq!(int_to_roman(3), "III");
        assert_eq!(int_to_roman(4), "IV");
        assert_eq!(int_to_roman(5), "V");
        assert_eq!(int_to_roman(9), "IX");
        assert_eq!(int_to_roman(12), "XII");
        assert_eq!(int_to_roman(16), "XVI");
        assert_eq!(int_to_roman(58), "LVIII"); // L = 50, V= 5, III = 3
        assert_eq!(int_to_roman(1994), "MCMXCIV"); // M = 1000, CM = 900, XC = 90 and IV = 4
    }
}
