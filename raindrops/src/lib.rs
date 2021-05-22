pub fn raindrops(n: u32) -> String {
    let mut sound: String = String::from("");

    if n % 3 == 0 {
        sound.push_str("Pling");
    }
    if n % 5 == 0 {
        sound.push_str("Plang");
    }
    if n % 7 == 0 {
        sound.push_str("Plong");
    };

    if sound.len() == 0 {
        return n.to_string();
    } else {
        return sound;
    }
}
