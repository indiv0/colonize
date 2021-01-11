debug:
	RUST_BACKTRACE=1 RUST_LOG=colonize=trace cargo run

release:
	RUST_BACKTRACE=1 RUST_LOG=colonize=debug cargo run --release

test:
	RUST_BACKTRACE=1 cargo test --all

wasm_release:
	CARGO_TARGET_DIR=./target-wasm cargo build --release --target wasm32-unknown-unknown --no-default-features --features wasm
	wasm-bindgen --out-dir target-wasm --target web target-wasm/wasm32-unknown-unknown/release/colonize.wasm
	wasm-opt -Oz target-wasm/colonize_bg.wasm -o target-wasm/colonize_bg_opt.wasm
	sed -i 's/_bg\.wasm/_bg_opt\.wasm/g' target-wasm/colonize.js
	cp target-wasm/colonize.js publish/colonize.js
	cp target-wasm/colonize_bg_opt.wasm publish/colonize_bg_opt.wasm

wasm_debug:
	CARGO_TARGET_DIR=./target-wasm cargo build --target wasm32-unknown-unknown --no-default-features --features wasm
	wasm-bindgen --out-dir target-wasm --target web target-wasm/wasm32-unknown-unknown/debug/colonize.wasm
	wasm-opt --debuginfo -Oz target-wasm/colonize_bg.wasm -o target-wasm/colonize_bg_opt.wasm
	sed -i 's/_bg\.wasm/_bg_opt\.wasm/g' target-wasm/colonize.js
	cp target-wasm/colonize.js publish/colonize.js
	cp target-wasm/colonize_bg_opt.wasm publish/colonize_bg_opt.wasm

styles:
	(cd www && NODE_ENV=production yarn run tailwindcss build style.css -o ../publish/style.css)

styles_debug:
	(cd www && NODE_ENV=development yarn run tailwindcss build style.css -o ../publish/style.css)

serve:
	(cd publish && python3 -m http.server)

deploy_debug:
	(cd publish && aws s3 sync . s3://dev.colonize.rs/)

deploy:
	(cd publish && aws s3 sync . s3://colonize.rs/)
