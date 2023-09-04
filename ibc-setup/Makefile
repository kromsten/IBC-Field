COMPOSE ?= docker-compose

build-gaia:
		@docker buildx build --load --build-context --build-context -t gaia-node -f dockerbuilds/Dockerfile.gaia --build-arg BINARY=gaiad .

build-secret:
		@$(COMPOSE) build localsecret

build-hermes:
		@docker build -f dockerbuilds/Dockerfile.hermes -t hermes:1.3.0-a285c01 .


build-all: build-gaia build-neutron build-hermes 

start-cosmopark: build-secret build-gaia build-hermes
		@$(COMPOSE) up -d

start-cosmopark-attached: build-secret build-gaia build-hermes
		@$(COMPOSE) up


start-cosmopark-no-rebuild:
		@$(COMPOSE) up -d

stop-cosmopark:
		@$(COMPOSE) down -t0 --remove-orphans -v

clean:
		@echo "Removing previous testing data"
		-@docker volume rm akashathon-data
