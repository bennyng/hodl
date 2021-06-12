SERVER_USER=deployer
SERVER_IP=`cat ./.server_ip`

.PHONY: dev image-build image-save image-copy remote-rm remote-run

dev:
	cargo run

image-build:
	docker build -t hodl-backend ./

image-save:
	docker save -o ./hodl-backend.tar.gz hodl-backend

image-copy:
	scp -i ./deploykey ./hodl-backend.tar.gz ${SERVER_USER}@$(SERVER_IP):/home/${SERVER_USER}/ && \
	ssh -i ./deploykey ${SERVER_USER}@$(SERVER_IP) "docker load -i /home/${SERVER_USER}/hodl-backend.tar.gz"

remote-rm:
	ssh -i ./deploykey ${SERVER_USER}@$(SERVER_IP) "docker stop hodl-backend || true" && \
	ssh -i ./deploykey ${SERVER_USER}@$(SERVER_IP) "docker rm hodl-backend || true"

remote-run:
	ssh -i ./deploykey ${SERVER_USER}@$(SERVER_IP) "docker run -dit --name hodl-backend --network caddy_default -p 8000 hodl-backend"

ssh:
	ssh -i ./deploykey ${SERVER_USER}@$(SERVER_IP)

deploy: image-build image-save image-copy remote-rm remote-run