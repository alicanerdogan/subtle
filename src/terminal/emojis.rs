use regex::Regex;

fn is_code_valid(isocode: &str) -> bool {
    let re = Regex::new(r"(?i)^[a-z]{2}$").unwrap();
    re.is_match(isocode)
}

fn get_utf16(c: &char) -> u16 {
    let mut b = [0];
    c.encode_utf16(&mut b);

    b[0]
}

fn get_uppercase(c: &char) -> char {
    c.to_uppercase().next().unwrap()
}

fn to_utf8(codepoint: u32) -> Vec<u8> {
    let mut ut8_bytes: Vec<u8> = Vec::with_capacity(4);
    if codepoint <= 0x7F {
        ut8_bytes.push(codepoint as u8);
        return ut8_bytes;
    }
    if codepoint <= 0x7FF {
        ut8_bytes.push((0xC0 | (codepoint >> 6)) as u8);
        ut8_bytes.push((0x80 | (codepoint & 0x3F)) as u8);
        return ut8_bytes;
    }
    if codepoint <= 0xFFFF {
        ut8_bytes.push((0xE0 | (codepoint >> 12)) as u8);
        ut8_bytes.push((0x80 | ((codepoint >> 6) & 0x3F)) as u8);
        ut8_bytes.push((0x80 | (codepoint & 0x3F)) as u8);
        return ut8_bytes;
    }
    if codepoint <= 0x10FFFF {
        ut8_bytes.push((0xF0 | (codepoint >> 18)) as u8);
        ut8_bytes.push((0x80 | ((codepoint >> 12) & 0x3F)) as u8);
        ut8_bytes.push((0x80 | ((codepoint >> 6) & 0x3F)) as u8);
        ut8_bytes.push((0x80 | (codepoint & 0x3F)) as u8);
        return ut8_bytes;
    }
    return ut8_bytes;
}

pub fn get_flag_emoji(isocode: &str) -> Option<String> {
    const OFFSET: u32 = 127397;

    if !is_code_valid(isocode) {
        return None;
    }

    let isocode_as_str = String::from(isocode);
    let mut char_iter = isocode_as_str.chars();

    let mut bytes: Vec<u8> = Vec::with_capacity(8);

    loop {
        let c = char_iter.next();

        if c == None {
            break;
        }

        let c = c.unwrap();
        let c = get_uppercase(&c);
        let char_utf16 = get_utf16(&c);
        let char_bytes = to_utf8(OFFSET + char_utf16 as u32);
        for byte in char_bytes.iter() {
            bytes.push(*byte);
        }
    }

    match String::from_utf8(bytes) {
        Ok(emoji) => Some(emoji),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn regex() {
        assert!(super::is_code_valid("tr"));
        assert!(super::is_code_valid("us"));
        assert!(super::is_code_valid("uk"));
        assert!(super::is_code_valid("GB"));
        assert!(super::is_code_valid("CN"));
        assert!(super::is_code_valid("aE"));
        assert_eq!(super::is_code_valid("aaa"), false);
        assert_eq!(super::is_code_valid("123"), false);
    }

    #[test]
    fn emoji() {
        assert_eq!(super::get_flag_emoji("tr").unwrap(), String::from("🇹🇷"));
        assert_eq!(super::get_flag_emoji("us").unwrap(), String::from("🇺🇸"));
        assert_eq!(super::get_flag_emoji("US").unwrap(), String::from("🇺🇸"));
        assert_eq!(super::get_flag_emoji("abc"), None);
        assert_eq!(super::get_flag_emoji("123"), None);

        println!("{}", console::Emoji("🇺🇸", "US"))
    }
}
