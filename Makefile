log:
	wrangler tail | jq '.'

pub:
	wrangler build
	wrangler publish

test-get:
	curl https://cloudflare-rust-kv-example.paul-asvb.workers.dev/kv/mykey

test-post:
	curl -X POST https://cloudflare-rust-kv-example.paul-asvb.workers.dev/kv/mykey \
   -H 'Content-Type: application/json' \
   -d '{"k1":"v1","k2":"v2"}'

test: test-post test-get
	echo "tested"