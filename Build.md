## Client:
In the ``client`` directory

``
wasm-pack build --target web --out-name package --out-dir ../server/www/static
``

or 

``
wasm-pack build --target web --out-name package --out-dir ../server/www/static --dev
``


## Server:
In the ``server`` directory:

``
cargo run --release
``

or

``
cargo run
``