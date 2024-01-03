// https://www.codewars.com/kata/52eb114b2d55f0e69800078d

// solution 1: 1332ms
struct Cipher {
    map1: String,
    map2: String,
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        Cipher {
            map1: map1.to_owned(),
            map2: map2.to_owned(),
        }
    }

    fn encode(&self, string: &str) -> String {
        let mut result = String::new();
        for c in string.chars() {
            let index = self.map1.find(c);
            match index {
                Some(i) => result.push(self.map2.chars().nth(i).unwrap()),
                None    => result.push(c),
            }
        }
        result
    }

    fn decode(&self, string: &str) -> String {
        let mut result = String::new();
        for c in string.chars() {
            let index = self.map2.find(c);
            match index {
                Some(i) => result.push(self.map1.chars().nth(i).unwrap()),
                None    => result.push(c),
            }
        }
        result 
    }
}