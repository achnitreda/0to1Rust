pub fn score(input : &str) -> u64 {
    let s = input.to_uppercase();

    let mut n = 0;
    for c in s.chars() {
        match c {
            'A'|'E'|'I'|'O'|'U'|'L'|'N'|'R'|'S'|'T' => n+=1,
            'G'|'D' => n+=2,
            'B'|'C'|'M'|'P' => n+=3,
            'F'|'H'|'V'|'W'|'Y' => n+=4,
            'K' => n+=5,
            'J'|'X' => n+=8,
            'Q'|'Z' => n+=10,
            _ => n+=0,
        }
    }

    n
}