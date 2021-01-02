debug:
	RUST_BACKTRACE=1 RUST_LOG=colonize=trace cargo run

release:
	RUST_BACKTRACE=1 RUST_LOG=colonize=trace cargo run --release

test:
	RUST_BACKTRACE=1 cargo test --all

wasm_release:
	CARGO_TARGET_DIR=./target-wasm cargo build --release --target wasm32-unknown-unknown --no-default-features --features wasm
	wasm-bindgen --out-dir target-wasm --target web target-wasm/wasm32-unknown-unknown/release/colonize.wasm
	wasm-opt -Oz target-wasm/colonize_bg.wasm -o target-wasm/colonize_bg_opt.wasm
	sed -i 's/_bg\.wasm/_bg_opt\.wasm/g' target-wasm/colonize.js

wasm_debug:
	CARGO_TARGET_DIR=./target-wasm cargo build --target wasm32-unknown-unknown --no-default-features --features wasm
	wasm-bindgen --out-dir target-wasm --target web target-wasm/wasm32-unknown-unknown/debug/colonize.wasm
	wasm-opt --debuginfo -Oz target-wasm/colonize_bg.wasm -o target-wasm/colonize_bg_opt.wasm
	sed -i 's/_bg\.wasm/_bg_opt\.wasm/g' target-wasm/colonize.js

serve:
	python3 -m http.server

deploy_debug:
	aws s3 cp index.html s3://dev.colonize.rs/index.html
	aws s3 cp target-wasm/colonize.js s3://dev.colonize.rs/target/colonize.js
	aws s3 cp target-wasm/colonize_bg_opt.wasm s3://dev.colonize.rs/target/colonize_bg_opt.wasm

deploy:
	aws s3 cp index.html s3://colonize.rs/index.html
	aws s3 cp target-wasm/colonize.js s3://colonize.rs/target/colonize.js
	aws s3 cp target-wasm/colonize_bg_opt.wasm s3://colonize.rs/target/colonize_bg_opt.wasm

