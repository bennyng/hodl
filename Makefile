docker-build-push: docker-build-web docker-build-backend

docker-build-push-web:
	make -C ./web docker-build && \
	make -C ./web docker-push && \
	kubectl -n hodl rollout restart deployment hodl-web

docker-build-push-backend:
	make -C ./backend docker-build && \
	make -C ./backend docker-push && \
	kubectl -n hodl rollout restart deployment hodl-backend
		
apply-infra:
	kubectl apply -k infra/overlays/production
