IMAGE=ledger
REGISTRY_URL=gcr.io/sousandrei
VERSION=$(shell git rev-parse --short=7 HEAD)


build:
	docker build . -t ${IMAGE}

push:
	docker tag ${IMAGE} ${REGISTRY_URL}/${IMAGE}:${VERSION}
	docker push ${REGISTRY_URL}/${IMAGE}:${VERSION}

deploy: build push
	helm upgrade ledger ./chart --namespace ledger --set image=${VERSION}