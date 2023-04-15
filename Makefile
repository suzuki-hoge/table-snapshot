dev:
	yarn tauri dev

test:
	@cd src-tauri && cargo test

fix:
	@yarn lint
	@yarn format
	@cd src-tauri && cargo +nightly fmt
	@cd src-tauri && cargo fix --allow-dirty --allow-staged
	@cd src-tauri && cargo clippy --fix --allow-dirty --allow-staged
	@cd src-tauri && cargo test

up:
	docker compose up --detach

down:
	docker compose down --volumes

mysql80:
	docker compose exec mysql80 mysql -h localhost -u user -ppassword table-snapshot
