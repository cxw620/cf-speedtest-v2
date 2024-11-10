# cf-speedtest-v2

A simple Cloudflare Workers programme.

## Develop

Make sure nodejs is installed and wasm-opt exists in your path.

Clone the repo then `npx wrangler dev`.

See `wrangler`'s docs for more details.

## Deploy

For Cloudflare Workers, just run `npx wrangler deploy`.

To build locally, run `cargo build --package cf-speedtest-backend --release` and goto `target/release` to get the executable file.

## License

MIT
