# Default docker-compose command
COMPOSE = docker-compose

# Run the application
run:
	$(COMPOSE) exec pos_backend cargo run
	@echo ""

# Run migrations
migrate:
	$(COMPOSE) exec pos_backend cargo run migrate
	@echo ""


# Seed all tables
seed-all:
	$(COMPOSE) exec pos_backend cargo run seed --table all
	@echo ""

# Seed a specific table
seed:
	@if [ -z "$(TABLE)" ]; then \
		echo "Error: TABLE is not set. Use 'make seed TABLE=<table_name>'."; \
		exit 1; \
	fi
	$(COMPOSE) exec pos_backend cargo run seed --table $(TABLE)
	@echo ""

# Generate entities using sea-orm-cli
entities:
	sea-orm-cli generate entity -o src/entities
	@echo ""

# Help command
help:
	@echo "Available commands:"
	@echo "  make run                - Run the application"
	@echo "  make migrate            - Run migrations"
	@echo "  make seed-all           - Seed all tables"
	@echo "  make seed TABLE=<table> - Seed a specific table"
	@echo "  make generate-entities  - Generate entities using sea-orm-cli"

# Declare phony targets
.PHONY: run migrate seed-all seed generate-entities help