# ============================
#        CONFIGURATION
# ============================

VERSION        := $(shell git describe --tags --abbrev=0)
DOCKER_TAG     := $(shell date +%Y%m%d)
IMAGES         := rust-app

BUILDSTAMP_FILE := .podman-build-flag
BUILDSTAMPS     := $(addsuffix /$(BUILDSTAMP_FILE),$(IMAGES))

# ============================
#          TARGETS
# ============================

.PHONY: all clean compile release install

all: $(BUILDSTAMPS)

# Chaque buildstamp dépend de tous les fichiers du dossier (sauf lui-même)
%/$(BUILDSTAMP_FILE): $(filter-out %/$(BUILDSTAMP_FILE),$(wildcard %/*))
	$(call docker_build,$(@D))
	touch $@

clean:
	$(foreach img,$(IMAGES),$(call docker_clean,$(img));)
	rm -f $(BUILDSTAMPS)

compile: all

release:
	$(foreach img,$(IMAGES),$(call docker_release,$(img));)

install:
	@echo "Installation non implémentée"
	# podman run --detach --name rust-app jobjects/rust-app:${VERSION}-$(DOCKER_TAG)

# ============================
#        COMMAND TEMPLATES
# ============================

define docker_build
	@echo "🔨 Build image: $(1)  version: ${VERSION}-$(DOCKER_TAG)"
	@podman build --tag jobjects/$(1):${VERSION}-$(DOCKER_TAG) $(1)
	@podman tag jobjects/$(1):${VERSION}-$(DOCKER_TAG) jobjects/$(1):latest
	@podman tag jobjects/$(1):${VERSION}-$(DOCKER_TAG) jobjects/$(1):${VERSION}
endef

define docker_release
	@echo "🚀 Release image: $(1) version: ${VERSION}"
	@podman build --tag jobjects/$(1):${VERSION} $(1)
	@podman tag jobjects/$(1):${VERSION} docker.io/mpatron/$(1):${VERSION}
	@podman tag jobjects/$(1):${VERSION} docker.io/mpatron/$(1):latest
	@podman push docker.io/mpatron/$(1):${VERSION}
	@podman push docker.io/mpatron/$(1):latest
endef

define docker_clean
	@echo "🧹 Clean image: $(1)"
	@podman rmi --force --ignore jobjects/$(1):${VERSION}-$(DOCKER_TAG)
	@podman rmi --force --ignore jobjects/$(1):latest
	@podman rmi --force --ignore jobjects/$(1):${VERSION}
endef
