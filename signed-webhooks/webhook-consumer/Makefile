.PHONY build:
build:
	source ./.venv/bin/activate; \
	pip install -r requirements.txt; \
	componentize-py -d wit -w verification componentize -m spin_sdk=spin-http app -o app.wasm; \
	deactivate
	wasm-tools compose -d ./../hmac/target/wasm32-wasi/release/hmac.wasm ./app.wasm -o ./composed.wasm 
