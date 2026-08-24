BINARY				:= ma
CARGO    			:= cargo
RELEASE  			:= target/release/$(BINARY)
DEBUG    			:= target/debug/$(BINARY)
CLIPPY_STRICT := --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery
SRCS					:= Cargo.toml Cargo.lock src/i18n.yaml $(shell find src -name '*.rs')
PREFIX   			?= $(HOME)/.local/bin
PUBLISH  			:= ma:bin/
PUBLISH_SH    := .publish.sh
RUN_ARGS			?=
DOCKER			?= docker
DOCKER_COMPOSE	?= docker compose
DOCKER_IMAGE	?= bahner/ma
DOCKER_TAG		?= latest
MA_VERSION		?=
DEB_CHANGED_BY	:= $(shell if [ -n "$$DEBFULLNAME" ] && [ -n "$$DEBEMAIL" ]; then printf '%s <%s>' "$$DEBFULLNAME" "$$DEBEMAIL"; else printf '%s' 'ma maintainers <maintainers@ma.invalid>'; fi)
DEBSIGN_KEYID	?=
DEB_SIGN_ARGS	:= $(if $(DEBSIGN_KEYID),-k$(DEBSIGN_KEYID))

.PHONY: all clean deb distclean docker docker-image docker-push install lint publish release test

all: $(BINARY)

lint:
	$(CARGO) clippy -- -D warnings
	$(CARGO) fmt --check
	mdl *.md

test:
	$(CARGO) clippy $(CLIPPY_STRICT)
	$(CARGO) test --all-features

deb:
	dpkg-buildpackage -b -nc -d $(DEB_SIGN_ARGS) --changes-option="--changed-by=$(DEB_CHANGED_BY)"

docker:
	$(DOCKER_COMPOSE) up

docker-image:
	$(DOCKER) build --tag $(DOCKER_IMAGE):$(DOCKER_TAG) .

docker-push: docker-image
	$(DOCKER) push $(DOCKER_IMAGE):$(DOCKER_TAG)

# Publish all i18n/*.ftl files to IPFS and write the resulting CIDs to
# src/i18n.yaml.  Requires `ipfs` (Kubo) and `jq` to be available.
# This file is a build input: `make release` will rebuild the binary
# whenever any FTL file changes.
src/i18n.yaml: $(wildcard i18n/*.ftl)
	@set -e; \
	dag='{}'; \
	for f in i18n/*.ftl; do \
		code=$$(basename $$f .ftl); \
		cid=$$(ipfs add -q --cid-version 1 "$$f"); \
		dag=$$(printf '%s' "$$dag" | jq --arg k "$$code" --arg v "$$cid" '. + {($$k): {"/": $$v}}'); \
	done; \
	lang_cid=$$(printf '%s' "$$dag" | ipfs dag put --input-codec dag-json --store-codec dag-cbor); \
	{ \
		printf 'i18n_cid: %s\n' "$$lang_cid"; \
		printf 'langs:\n'; \
		printf '%s' "$$dag" | jq -r 'to_entries[] | "  " + .key + ": " + .value["/"]'; \
	} > src/i18n.yaml; \
	echo "Written src/i18n.yaml (i18n_cid=$$lang_cid)"

$(RELEASE): $(SRCS)
	$(CARGO) build --release

clean:
	$(CARGO) clean

install: $(RELEASE)
	mkdir -p $(PREFIX)
	install -m 0755 $(RELEASE) $(PREFIX)/$(BINARY)

publish: $(RELEASE)
	scp $(RELEASE) $(PUBLISH)
	test -f $(PUBLISH_SH) && bash $(PUBLISH_SH)

release:
	@test -n "$(MA_VERSION)" || { echo "MA_VERSION is required (for example: make release MA_VERSION=0.0.1)" >&2; exit 2; }
	$(CARGO) build --release
	cargo release $(MA_VERSION) --no-confirm --no-publish --execute
	scp $(RELEASE) $(PUBLISH)
	test -f $(PUBLISH_SH) && bash $(PUBLISH_SH)
	$(MAKE) docker-push DOCKER_TAG=$(MA_VERSION)

distclean: clean
	rm -rf target
	rm -rf Cargo.lock src/i18n.yaml
