asmjs:
	@cargo build --release -p capi --target=asmjs-unknown-emscripten
	@cp ../../target/asmjs-unknown-emscripten/release/livesplit*.js* livesplit.js
	@cat exports.js >> livesplit.js

node: asmjs
	(cd ../bind_gen && cargo run)
	cp ../bindings/emscripten/livesplit_core.js node/index.js
	cp livesplit.js node/livesplit_core.js
	cp ../../README.md node/README.md

clean:
	@cargo clean
	@rm livesplit*.js*
