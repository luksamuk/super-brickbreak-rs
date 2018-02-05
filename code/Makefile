name=super-brickbreak-rs



.PHONY: folder wasm webstart ../index.html

all: ../index.html ../$(name).wasm




../index.html: static/index.html
	cp static/* ../
	find ../index.html -type f -exec sed -i 's/js\/app.js/$(name).js/g' {} +

../$(name).wasm: wasm
	cp target/wasm32-unknown-unknown/release/*.wasm ../
	cp target/wasm32-unknown-unknown/release/*.js ../




wasm: src/main.rs
	cargo +nightly web build --target-webasm --release





webstart:
	cargo +nightly web start --target-webasm --release
