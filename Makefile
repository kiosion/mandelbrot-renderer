.PHONY: build, dev, serve, clean

build-wasm:
	@echo "Building WASM..."
	@cd ./wasm && wasm-pack build

build: build-wasm
	@cd ./ui && pnpm install && pnpm run build
	@echo "Done! Run 'make serve' to start the web UI"

dev:
	@echo "Building..."
	@cd ./wasm && wasm-pack build
	@cd ./ui && pnpm install && pnpm run dev

serve:
	@node ./ui/build

clean:
	@rm -rf ./wasm/pkg
	@rm -rf ./ui/build
