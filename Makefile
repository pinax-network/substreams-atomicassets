.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run: build
	substreams run -e eos.substreams.pinax.network:443 map_events -s 338947637 -t +1

.PHONY: gui
gui: build
	substreams gui -e eos.substreams.pinax.network:443 map_events -s 329693282 -t 329871503

.PHONY: graph_out
graph_out: build
	substreams run -e eos.substreams.pinax.network:443 substreams.yaml graph_out -s 338947637 -t +1