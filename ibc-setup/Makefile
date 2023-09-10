COMPOSE ?= docker-compose

start-cosmopark: @$(COMPOSE) up --build -d
start-cosmopark-attached: @$(COMPOSE) up --build
start-cosmopark-no-rebuild: @$(COMPOSE) up -d
start-cosmopark-no-rebuild-attached: @$(COMPOSE) up

stop-cosmopark: @$(COMPOSE) down -t0 --remove-orphans -v

clean: @echo "Removing previous testing data"
		-@docker volume rm akashathon-data

