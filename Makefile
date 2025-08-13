dev:
	cargo run -p ci -- lints
	cargo run -p ci -- test
	cargo run -p ci -- doc
	cargo run -p ci -- compile