fn scytale_cipher(message: String, i: u32) -> String {
    // if message == "" {
        
    // }
	let len = message.len();
    let cols = (len as f32 / i as f32).ceil() as usize;
    let mut result = Vec::new();

    if len % 2 == 0 {
        result = vec![' '; len];
    } else {
        result = vec![' '; len + 1];

    } 

for col in 0..cols {
    let start_index = col * i as usize;

    let mut chars: Vec<char> = message.chars().skip(start_index).take(i as usize).collect();
    chars.resize(i as usize, ' ');

    for (i, c) in chars.into_iter().enumerate() {
        let index = i * cols + col;
        result[index] = c;
    }
}

result.iter().collect()
}



