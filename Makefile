pub:
	wrangler build
	wrangler publish

test-post:
	curl -X POST https://cloudflare-rust-kv-example.paul-asvb.workers.dev/kv/sdfg
