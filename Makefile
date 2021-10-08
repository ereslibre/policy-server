HYPERFINE := $(shell command -v hyperfine 2> /dev/null)

.PHONY: build
build:
	cargo build --release

.PHONY: fmt
fmt:
	cargo fmt --all -- --check

.PHONY: lint
lint:
	cargo clippy -- -D warnings

.PHONY: test
test: fmt lint
	cargo test --workspace

.PHONY: clean
clean:
	cargo clean

.PHONY: tag
tag:
	@git tag "${TAG}" || (echo "Tag ${TAG} already exists. If you want to retag, delete it manually and re-run this command" && exit 1)
	@git-chglog --output CHANGELOG.md
	@git commit -m 'Update CHANGELOG.md' -- CHANGELOG.md
	@git tag -f "${TAG}"

# REMOVE ME (ereslibre)
.PHONY: metrics
metrics:
	(docker ps -aq | xargs docker rm -f) || true
	docker run -d --rm \
		--name jaeger \
		-p14250:14250 \
		-p16686:16686 \
		jaegertracing/all-in-one:latest
	docker run -d --rm \
		-p 4317:4317 \
    -p 8888:8888 \
		-v $(PWD)/otel-collector-minimal-config.yaml:/etc/otel/config.yaml:ro \
		otel/opentelemetry-collector-contrib-dev:latest \
		--log-level debug \
		--config /etc/otel/config.yaml
	docker run -d --rm \
	  -p 9090:9090 \
	  -v $(PWD)/prometheus.yml:/etc/prometheus/prometheus.yml \
	  prom/prometheus
	cargo run --release -- \
	  --policies policies.yml \
	  --workers 2 \
	  --log-fmt otlp \
	  --log-level debug \
		--enable-metrics
