#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_correct_isin_check_digit() {
        assert_eq!(calculate_isin_check_digit(String::from("US0378331005")), Some(5));
        assert_eq!(calculate_isin_check_digit(String::from("AU0000XVGZA3")), Some(3));
        assert_eq!(calculate_isin_check_digit(String::from("GB00BH4HKS39")), Some(9));
        assert_eq!(calculate_isin_check_digit(String::from("GB0002634946")), Some(6));
        assert_eq!(calculate_isin_check_digit(String::from("US037#331005")), None);
    }

    #[test]
    fn generate_correct_cusip_check_digit() {
        assert_eq!(calculate_cusip_check_digit(String::from("037833100")), Some(0));
        assert_eq!(calculate_cusip_check_digit(String::from("17275R102")), Some(2));
        assert_eq!(calculate_cusip_check_digit(String::from("38259P508")), Some(8));
        assert_eq!(calculate_cusip_check_digit(String::from("594918104")), Some(4));
        assert_eq!(calculate_cusip_check_digit(String::from("68389X105")), Some(5));
        assert_eq!(calculate_cusip_check_digit(String::from("68389$105")), None);
    }

    #[test]
    fn generate_correct_sedol_check_digit() {
        assert_eq!(calculate_sedol_check_digit(String::from("0263494")), Some(4));
        assert_eq!(calculate_sedol_check_digit(String::from("B09LQS3")), Some(3));
        assert_eq!(calculate_sedol_check_digit(String::from("B677469")), Some(9));
        assert_eq!(calculate_sedol_check_digit(String::from("0981536")), Some(6));
        assert_eq!(calculate_sedol_check_digit(String::from("A023494")), None);
    }

    #[test]
    fn generate_correct_figi_check_digit() {
        assert_eq!(calculate_figi_check_digit(String::from("BBG000BLNQ16")), Some(6));
    }

    #[test]
    fn generate_isin_from_sedol() {
        assert_eq!(convert_sedol_to_gb_isin(String::from("B09LQS3")), Some(String::from("GB00B09LQS34")));
        assert_eq!(convert_sedol_to_gb_isin(String::from("A023494")), None);
    }

    #[test]
    fn generate_isin_from_cusip() {
        assert_eq!(convert_cusip_to_us_isin(String::from("037833100")), Some(String::from("US0378331005")));
        assert_eq!(convert_cusip_to_us_isin(String::from("037833$00")), None);
    }
}

pub fn calculate_isin_check_digit(isin: String) -> Option<u8> {
    let last_char_index = isin.len() - 1;
    let mut isin_digits = String::new();

    for c in isin.to_uppercase()[..last_char_index].chars() {
        match c {
            'A'...'Z' => isin_digits.push_str(&((c as u32 - 55).to_string())[..]),
            '0'...'9' => isin_digits.push_str(&(c.to_digit(10).unwrap().to_string())[..]),
            _ => return None        // exit early because we have a char that would not be in an isin
        }
    }

    let (mut l, mut r) = (Vec::new(), Vec::new());

    for (i, c) in isin_digits.chars().enumerate() {
        if i % 2 == 0 { l.push(c.to_digit(10).unwrap()); }
        if (i + 1) % 2 == 0 { r.push(c.to_digit(10).unwrap()); }
    }

    fn get_digits(double_digits: Vec<u32>, single_digits: &mut Vec<u32>) -> Vec<u32> {
        let d: Vec<u32> = double_digits.iter().map(|x| x * 2).collect();
        let mut s = String::new();
        
        for v in &d {
            s.push_str(&(v.to_string())[..]);
        }
        
        let chars = s.chars().collect::<Vec<char>>();
        
        let mut digits: Vec<u32> = chars.iter().map(|x| x.to_digit(10).unwrap()).collect();
        digits.append(single_digits);
        
        digits
    }

    let checksum_digits: Vec<u32> = if isin_digits.len() % 2 == 1 {
        get_digits(l, &mut r)
    } else {
        get_digits(r, &mut l)
    };

    Some((10 - (checksum_digits.iter().sum::<u32>() % 10) % 10) as u8)
}

pub fn calculate_cusip_check_digit(cusip: String) -> Option<u8> {
    let last_char = cusip.len() - 1;
    
    let mut cusip_digits = Vec::<u32>::new();
    
    for (i, c) in cusip.to_uppercase()[..last_char].chars().enumerate() {
    
        match i {
            0...7 if (i + 1) % 2 == 0 => 
                match c {
                    '0'...'9' => cusip_digits.push(c.to_digit(10).unwrap() * 2),
                    'A'...'Z' => cusip_digits.push((c as u32 - 55) * 2),
                    '*' => cusip_digits.push(36 * 2),
                    '@' => cusip_digits.push(37 * 2),
                    '#' => cusip_digits.push(38 * 2),
                    _ => return None,
                },
            0...7 if (i + 1) % 2 == 1 =>
                match c {
                    '0'...'9' => cusip_digits.push(c.to_digit(10).unwrap()),
                    'A'...'Z' => cusip_digits.push(c as u32 - 55),
                    '*' => cusip_digits.push(36),
                    '@' => cusip_digits.push(37),
                    '#' => cusip_digits.push(38),
                    _ => return None,
                },
            _ => return None,
        }
    }
    
    let sum = cusip_digits.iter().map(|x| x / 10 + x % 10).sum::<u32>();

    Some(((10 - (sum % 10)) % 10) as u8)
}

pub fn calculate_sedol_check_digit(sedol: String) -> Option<u8> {
    let sedol_weights = vec![1u32, 3u32, 1u32, 7u32, 3u32, 9u32];
    let last_char = sedol.len() - 1;
    let mut sedol_digits = Vec::<u32>::new();

    for c in sedol.to_uppercase()[..last_char].chars() {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' => return None,
            '0'...'9' => sedol_digits.push(c.to_digit(10).unwrap()),
            'B'...'Z' => sedol_digits.push(c as u32 - 55),
            _ => return None
        }
    }

    let weighted_digits = sedol_digits.iter().zip(sedol_weights.iter());
    
    let sum: u32 = weighted_digits.map(|(a, b)| a * b).sum();

    Some(((10 - (sum % 10)) % 10) as u8)
}

pub fn calculate_figi_check_digit(figi: String) -> Option<u8> {
    let last_char = figi.len() - 1;
    
    let mut figi_digits = Vec::<u32>::new();
    
    for (i, c) in figi[..last_char].chars().enumerate() {
        match i {
            0...10 if (i + 1) % 2 == 0 =>
                match c {
                    '0'...'9' => figi_digits.push(c.to_digit(10).unwrap() * 2),
                    'A'...'Z' => figi_digits.push((c as u32 - 55) * 2),
                    _ => (),
                }
            0...10 if (i + 1) % 2 == 1 =>
                match c {
                    '0'...'9' => figi_digits.push(c.to_digit(10).unwrap()),
                    'A'...'Z' => figi_digits.push(c as u32 - 55),
                    _ => (),
                }
            _ => ()
        }
    }
    
    let digit_string = figi_digits.iter().map(|x| x.to_string()).collect::<Vec<String>>().concat();
    let digit_chars: Vec<char> = digit_string.chars().collect();
    let digits: Vec<u32> = digit_chars.iter().map(|x| x.to_digit(10).unwrap()).collect();
    
    let sum: u32 = digits.iter().sum();

    Some(((10 - (sum % 10)) % 10) as u8)
}

pub fn convert_sedol_to_gb_isin(sedol: String) -> Option<String> {
    convert_sedol_to_isin(sedol, String::from("GB00"))
}

pub fn convert_sedol_to_ie_isin(sedol: String) -> Option<String> {
    convert_sedol_to_isin(sedol, String::from("IE00"))
}

pub fn convert_cusip_to_us_isin(cusip: String) -> Option<String> {
    convert_cusip_to_isin(cusip, String::from("US"))
}

pub fn convert_cusip_to_ca_isin(cusip: String) -> Option<String> {
    convert_cusip_to_isin(cusip, String::from("CA"))
}

fn convert_sedol_to_isin(sedol: String, prefix: String) -> Option<String> {
    let isin_prefix = format!("{}{}", prefix, &sedol);  
    
    match calculate_sedol_check_digit(sedol) {
        Some(_) => 
            match calculate_isin_check_digit(format!("{}?", &isin_prefix)) {
                Some(cd) => Some(format!("{}{}", isin_prefix, cd)),
                None => None,
            },
        None => None
    }
}

fn convert_cusip_to_isin(cusip: String, prefix: String) -> Option<String> {
    let isin_prefix = format!("{}{}", prefix, &cusip);    

    match calculate_cusip_check_digit(cusip) {
        Some(_) => 
            match calculate_isin_check_digit(format!("{}?", &isin_prefix)) {
                Some(cd) => Some(format!("{}{}", isin_prefix, cd)),
                None => None,
            }
        None => None
    }
}
