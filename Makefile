build-image:
	docker build --target release . -t ogontaro/image-mirror:latest
	docker push ogontaro/image-mirror:latest
