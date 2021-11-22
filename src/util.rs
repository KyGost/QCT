use std::collections::HashMap;

pub(crate) fn get_bit_frequencies(
	string_freqs: HashMap<String, usize>,
	bits: usize,
	shots: usize,
) -> Vec<f64> {
	let cbits = string_freqs.iter().take(1).len(); // TODO: Clean
	string_freqs
		.into_iter()
		.fold(vec![0; cbits], |mut tots, (bits, occurs)| {
			for (pos, bit) in bits.chars().rev().enumerate() {
				if bit == '1' {
					tots[pos] += occurs;
				}
			}
			tots
		})
		.into_iter()
		.map(|total| total as f64 / shots as f64)
		.collect()
}

pub(crate) fn get_ints_from_frequencies(freqs: Vec<f64>, accuracy_factor: usize) -> Vec<i64> {
	freqs
		.into_iter()
		.map(|freq| (freq * accuracy_factor as f64) as i64)
		.collect()
}

pub(crate) fn chance_to_rot(chance: f64) -> f64 {
	((chance * -2.0) + 1.0).acos()
}
