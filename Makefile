build:
	@echo "Compiling injectdb (dev)..."
	uv run maturin develop

release:
	@echo "Compiling injectdb (release)..."
	uv run maturin build --release

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	uv cache clean
	rm -rf target/ dist/ *.egg-info
	rm -rf .venv/lib/python3.14/site-packages/injectdb*
	find . -type f -name "*.pyc" -delete
	find . -type d -name "__pycache__" -delete

rebuild: clean build
	@echo "Rebuild complete."

unittest: rebuild
	@echo "Running Rust tests..."
	cargo test --release

	@echo "Running Python tests..."
	uv run pytest

wildtest: rebuild
	@echo "Running wild tests (skips if dataset not present)..."
	cargo test --release --test test_wild -- --include-ignored --nocapture || true

update:
	@echo "Updating dependencies..."
	cargo update
	uv sync --all-groups

prune:
	git fetch -p && for branch in $$(git branch -vv | grep ': gone]' | awk '{print $$1}'); do git branch -D $$branch; done
