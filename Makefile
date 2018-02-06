name=super-brickbreak-rs
basefolder=docs


.PHONY: folder wasm webstart $(basefolder)/index.html

all: $(basefolder)/index.html $(basefolder)/$(name).wasm




$(basefolder)/index.html: static/index.html static/*.png
	cp static/* $(basefolder)
	find $(basefolder)/index.html -type f -exec sed -i 's/js\/app.js/$(name).js/g' {} +

$(basefolder)/$(name).wasm: wasm
	cp target/wasm32-unknown-unknown/release/*.wasm $(basefolder)
	cp target/wasm32-unknown-unknown/release/*.js $(basefolder)




wasm: src/main.rs
	cargo +nightly web build --target-webasm --release





webstart:
	cargo +nightly web start --target-webasm --release
