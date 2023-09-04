COMPOSE ?= docker-compose

build-secret:
		@$(COMPOSE) build localsecret

build-hermes:
		@docker build -f dockerbuilds/Dockerfile.hermes -t hermes:1.3.0-a285c01 .


build-all: build-secret build-hermes 


start-cosmopark: build-all
		@$(COMPOSE) up -d

start-cosmopark-attached: build-all
		@$(COMPOSE) up

start-cosmopark-no-rebuild:
		@$(COMPOSE) up -d

stop-cosmopark:
		@$(COMPOSE) down -t0 --remove-orphans -v

clean:
		@echo "Removing previous testing data"
		-@docker volume rm akashathon-data
