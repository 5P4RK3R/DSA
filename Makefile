# Clean the project
clean:
	cargo clean
	mdbook clean
	find . -type f -name "*.DS_Store" -delete
	find . -type f -name "*.log" -delete
	find . -type d -name 'target' -exec rm -rf {} +

run:
	cargo run

build:
	cargo build

doc:
	cargo doc
	mdbook build
	mdbook serve   
	chmod +x doc.sh
	./doc.sh