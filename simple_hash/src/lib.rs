
pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut map = HashMap::new();

    for word in words {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
	map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
	let mut result = 0;
	let count: usize  = 1;

	for (_, val) in frequency_count.iter() {
		if val == &count {
			result += 1;
		}
	}
	result
}
