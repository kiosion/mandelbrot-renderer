.PHONY: build, dev, serve, clean

compile-shader:
	@echo "Compiling GLSL shaders..."
	@cd ./wasm/src/shader && \
	glslc shader.comp -fshader-stage=compute -o shader.spv

build-wasm: compile-shader
	@echo "Building WASM..."
	@cd ./wasm && \
	RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build

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
