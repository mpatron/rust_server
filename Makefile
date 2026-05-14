# ============================
#        CONFIGURATION
# ============================

VERSION        := $(shell git describe --tags --abbrev=0 2>/dev/null || echo "0.1.0")
DOCKER_TAG     := $(shell date +%Y%m%d)
IMAGE_NAME     := rust-app
FULL_TAG       := $(VERSION)-$(DOCKER_TAG)

# Moteur de conteneur (Podman par défaut)
CONTAINER_ENGINE := podman

# ============================
#          TARGETS
# ============================
.PHONY: all build clean run stop help release

all: build ## Construit l'image par défaut

build: ## Construit l'image avec les tags latest et FULL_TAG
	@echo "--- 🔨 Construction de l'image Rust : $(IMAGE_NAME):$(FULL_TAG) ---"
	$(CONTAINER_ENGINE) build \
		--tag $(IMAGE_NAME):latest \
		--tag $(IMAGE_NAME):$(FULL_TAG) \
		--file Containerfile .

run: ## Lance le conteneur en mode interactif
	@echo "--- 🏃 Lancement du conteneur $(IMAGE_NAME):latest ---"
	$(CONTAINER_ENGINE) run --detach --name $(IMAGE_NAME) --rm -p 8080:8080 $(IMAGE_NAME):latest
	echo "curl -v http://127.0.0.1:8080 && echo"

stop: ## Arrête le conteneur
	@echo "--- 🛑 Arrêt du conteneur $(IMAGE_NAME):latest ---"
	$(CONTAINER_ENGINE) stop $(IMAGE_NAME) || true
	$(CONTAINER_ENGINE) rm $(IMAGE_NAME) || true

clean: ## Supprime les images créées
	@echo "--- 🧹 Nettoyage des images ---"
	$(CONTAINER_ENGINE) rmi $(IMAGE_NAME):latest $(IMAGE_NAME):$(FULL_TAG) || true

release: ## Tag et pousse l'image vers Docker Hub
	@echo "🚀 Release image: $(IMAGE_NAME) version: $(VERSION)"
	$(CONTAINER_ENGINE) tag $(IMAGE_NAME):latest docker.io/mpatron/$(IMAGE_NAME):$(VERSION)
	$(CONTAINER_ENGINE) tag $(IMAGE_NAME):latest docker.io/mpatron/$(IMAGE_NAME):latest
	$(CONTAINER_ENGINE) push docker.io/mpatron/$(IMAGE_NAME):$(VERSION)
	$(CONTAINER_ENGINE) push docker.io/mpatron/$(IMAGE_NAME):latest

help: ## Affiche cette aide
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'
