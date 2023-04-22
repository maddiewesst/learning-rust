pub fn scytale_cipher(message: String, i: u32) -> String {
    // if message == "" {
        
    // }
	let len = message.len();
    let rows = (len as f32 / i as f32).ceil() as usize;
    let mut result= vec![' '; rows * i as usize];

    // if len % 2 == 0 {
    //     result = vec![' '; len];
    // } else {
    //     result = vec![' '; len + 1];

    // } 

for row in 0..rows {
    let start_index = row * i as usize;

    let mut chars: Vec<char> = message.chars().skip(start_index).take(i as usize).collect();
    chars.resize(i as usize, ' ');



    for (i, c) in chars.into_iter().enumerate() {
        let index = i * rows + row;
        if index >= result.len() { 
        continue; 
        } else {
            result[index] = c;
        }
    }
}

let string: String = result.into_iter().collect();
string.trim().to_string()
}