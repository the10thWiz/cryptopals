//
// psuedorandom.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

pub trait RngStream {
	type Output: AsRef<[u8]>;
	/// rng implementation MUST fill all bytes for get_rng
	fn get_rng(&mut self) -> Self::Output;
}

pub struct Rc4 {
	i: usize,
	cur: [u8; 256],
}

impl Rc4 {
	pub fn new() -> Self {
		let mut sorted = [0u8; 256];
		for (i, e) in sorted.iter_mut().enumerate() {
			*e = i as u8;
		}
		Self {
			i: 0,
			cur: sorted,
		}
	}
}

impl RngStream for Rc4 {
	type Output = [u8; 256];
	fn get_rng(&mut self) -> [u8; 256] {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
