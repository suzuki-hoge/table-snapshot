dev:
	yarn tauri dev

sb:
	yarn storybook

test:
	@cd src-tauri && cargo test -- --test-threads=1

front-fix:
	@yarn lint
	@yarn format

back-fix:
	@cd src-tauri && cargo +nightly fmt
	@cd src-tauri && cargo fix --allow-dirty --allow-staged
	@cd src-tauri && cargo clippy --fix --allow-dirty --allow-staged
	@make test

fix:
	@make front-fix
	@make back-fix

up:
	docker compose up --detach

down:
	docker compose down --volumes

db:
	docker compose exec table-snapshot mysql -h localhost -u user -ppassword table-snapshot

mysql80:
	docker compose exec testdata-mysql80 mysql -h localhost -u user -ppassword testdata

gen:
	@cd src-tauri && cargo run --bin component-generator

