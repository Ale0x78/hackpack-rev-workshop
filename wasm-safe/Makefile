chal:
	wasm-pack build --release --target web
	cp index.html app/
	cp -r pkg/wasm_safe_bg.wasm app/pkg/wasm_safe_bg.wasm
	cp -r pkg/wasm_safe.js app/pkg/wasm_safe.js

archive:
	tar -cvzf wasm_safe.tar.gz pkg/ index.html Makefile

clean:
	rm wasm_safe.tar.gz

container:
	podman build . -t wasm-safe
	podman run -d -p 8703:8000 wasm-safe:latest  

kill:
	podman kill wasm-safe:latest  
	podman rm wasm-safe:latest  
