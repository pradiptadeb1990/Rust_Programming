pub fn raindrops(n: u32) -> String {
    let mut raindrop_sound =String::new();

    if n % 3 == 0 {raindrop_sound += "Pling".into()};
    if n % 5 == 0 {raindrop_sound += "Plang".into()};
    if n % 7 == 0 {raindrop_sound += "Plong".into()};

    if raindrop_sound.is_empty(){
        raindrop_sound = n.to_string();
    }
    raindrop_sound
}
