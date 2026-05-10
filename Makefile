# ============================
#        CONFIGURATION
# ============================

VERSION        := $(shell git describe --tags --abbrev=0 2>/dev/null || echo "0.1.0")
DOCKER_TAG     := $(shell date +%Y%m%d)
IMAGE_NAME     := rust-app
FULL_TAG       := $(VERSION)-$(DOCKER_TAG)

# Le fichier flag est maintenant à la racine
BUILDSTAMP_FILE := .podman-build-flag

# ============================
#          TARGETS
# ============================

.PHONY: all clean compile release install run stop

all: $(BUILDSTAMP_FILE)

# Dépend de tous les fichiers sources pour redéclencher le build si un fichier change
$(BUILDSTAMP_FILE): Containerfile src/*.rs tests/*.rs Cargo.toml
	@echo "🔨 Build image: $(IMAGE_NAME) version: $(FULL_TAG)"
	podman build -f Containerfile --tag jobjects/$(IMAGE_NAME):$(FULL_TAG) .
	podman tag jobjects/$(IMAGE_NAME):$(FULL_TAG) jobjects/$(IMAGE_NAME):latest
	podman tag jobjects/$(IMAGE_NAME):$(FULL_TAG) jobjects/$(IMAGE_NAME):$(VERSION)
	touch $@

compile: all

run:
	@echo "🏃 Running container: $(IMAGE_NAME)"
	podman run --detach --name $(IMAGE_NAME) --rm -p 8080:8080 jobjects/$(IMAGE_NAME):latest

stop:
	@echo "🛑 Stopping container: $(IMAGE_NAME)"
	podman stop $(IMAGE_NAME) || true
	podman rm $(IMAGE_NAME) || true

clean:
	@echo "🧹 Cleaning image and flags"
	podman rmi --force jobjects/$(IMAGE_NAME):$(FULL_TAG) 2>/dev/null || true
	podman rmi --force jobjects/$(IMAGE_NAME):latest 2>/dev/null || true
	rm -f $(BUILDSTAMP_FILE)

release:
	@echo "🚀 Release image: $(IMAGE_NAME) version: $(VERSION)"
	podman tag jobjects/$(IMAGE_NAME):latest docker.io/mpatron/$(IMAGE_NAME):$(VERSION)
	podman tag jobjects/$(IMAGE_NAME):latest docker.io/mpatron/$(IMAGE_NAME):latest
	podman push docker.io/mpatron/$(IMAGE_NAME):$(VERSION)
	podman push docker.io/mpatron/$(IMAGE_NAME):latest
