# Set the default target of this Makefile
.PHONY: all
all:: clippy test

.PHONY: check-features
check-features:
	cargo hack check --feature-powerset --no-dev-deps

.PHONY: clippy
clippy:
	cargo clippy --all-features --all-targets

.PHONY: test
test:
	cargo nextest run --all-features
	cargo test --doc

# Update vendored protos from aiy-apis master branch.
.PHONY: update-protos
update-protos:
	@rm -rf vendored/aiy-apis
	@rm -rf vendored/proto
	@git clone -q --depth=1 https://github.com/MystenLabs/sui-apis.git vendored/aiy-apis
	@mv vendored/aiy-apis/proto vendored/
	@rm -rf vendored/aiy-apis

# Update vendored protos from a local clone of aiy-apis.
# Example invocation:
#     make -C crates/aiy-rpc update-protos-local /path/to/aiy-apis
.PHONY: update-protos-local
update-protos-local:
	@if [ -z "$(filter-out $@,$(MAKECMDGOALS))" ]; then \
		echo "Error: Path not provided. Usage: make update-protos-local /path/to/aiy-apis"; \
		exit 1; \
	fi
	@if [ ! -d "$(filter-out $@,$(MAKECMDGOALS))/proto" ]; then \
		echo "Error: $(filter-out $@,$(MAKECMDGOALS))/proto does not exist"; \
		exit 1; \
	fi
	@rm -rf vendored/proto
	@cp -r "$(filter-out $@,$(MAKECMDGOALS))/proto" vendored/
	@echo "Updated protos from $(filter-out $@,$(MAKECMDGOALS))"

.PHONY: diff-protos
diff-protos:
	@rm -rf vendored/aiy-apis
	@git clone -q --depth=1 https://github.com/MystenLabs/sui-apis.git vendored/aiy-apis
	git diff --no-index vendored/aiy-apis/proto vendored/proto
	@rm -rf vendored/aiy-apis

.PHONY: proto
proto:
	cargo run -p proto-build

%:
	$(MAKE) -C ../.. $@
