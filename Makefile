# Clean the project
clean:
	find . -type f -name "*.pyc" -delete
	find . -type f -name "*.DS_Store" -delete
	find . -type f -name "*.log" -delete
	find . -type d -name "__pycache__" -delete
	find . -type d -name 'target' -exec rm -rf {} +

run:
	cargo run