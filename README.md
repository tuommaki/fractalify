# fractalify

Simple program that fractalifies images. This is a PoC to test capabilities of
[Krustlet](https://krustlet.dev/).

## Build Instructions

In order to build this, you need to install a WASI-enabled Rust toolchain:
```
$ rustup target add wasm32-wasi
```

To build `fractalify` specify target for build:
```
$ cargo build --release --target wasm32-wasi
```

Now you should have the WebAssembly module created in: `target/wasm32-wasi/release`:
```
$ file target/wasm32-wasi/release/fractalify.wasm 
target/wasm32-wasi/release/fractalify.wasm: WebAssembly (wasm) binary module version 0x1 (MVP)
```

Once you have WebAssembly module created, you can use
[wasm-to-oci](https://github.com/engineerd/wasm-to-oci) to wrap WASM module
into container image and push it to registry:
```
$ wasm-to-oci push target/wasm32-wasi/release/fractalify.wasm foobar.azurecr.io/fractalify:v0.1.0
INFO[0002] Pushed: foobar.azurecr.io/fractalify:v0.1.0
INFO[0002] Size: 3145689
INFO[0002] Digest: sha256:3edad97fff87437e27e2eb0e4eb698f25c415d351903c9b2dcbe643cdaabde0f
```

## Deploying on Krustlet

Deploying on Krustlet is pretty straightforward. It should be sufficient to
have a POD definition that pulls the OCI image and has node selector +
tolerations for Krustlet node and relevant volume mounts:

```
apiVersion: v1
kind: Pod
metadata:
  name: fractalify
  labels:
    app: fractalify
spec:
  containers:
    - image: <container registry>/fractalify:v0.1.0
      imagePullPolicy: Always
      name: fractalify
      args: ["fractalify", "/input_dir", "/output_dir"]
      volumeMounts:
      - name: input-dir
        mountPath: /input_dir
      - name: output-dir
        mountPath: /output_dir
  imagePullSecrets:
    - name: image-pull-secret # If needed
  nodeSelector:
    kubernetes.io/arch: "wasm32-wasi"
  tolerations:
    - key: "node.kubernetes.io/network-unavailable"
      operator: "Exists"
      effect: "NoSchedule"
    - key: "kubernetes.io/arch"
      operator: "Equal"
      value: "wasm32-wasi"
      effect: "NoExecute"
    - key: "kubernetes.io/arch"
      operator: "Equal"
      value: "wasm32-wasi"
      effect: "NoSchedule"
  volumes:
    - name: input-dir
      hostPath:
        path: /input_dir
    - name: output-dir
      hostPath:
        path: /output_dir
```
