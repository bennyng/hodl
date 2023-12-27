.PHONY: dev docker-build docker-push

dev:
	npm run dev
	
docker-build:
	docker buildx build \
		-f ./Dockerfile \
		-t hodl-web \
		--progress=plain \
		--load \
		./; \

docker-push:
	docker tag hodl-web docker.io/bennynwh/hodl-web:latest; \
    docker push docker.io/bennynwh/hodl-web:latest