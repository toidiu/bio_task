.DEFAULT_GOAL := all

APACHE_DIR=apache

all: updateRepo releaseService releaseWebsite

updateRepo:
	git pull

releaseService:
	pkill "bio_task"; \
	cd service && cargo build --release && \
	(./target/release/bio_task &)

releaseWebsite:
	cd website && \
	npm run build && \
	mkdir -p ${APACHE_DIR}/biotask && \
	mv dist/* ${APACHE_DIR}/biotask

.PHONY: webBuildAndRelease





.PHONY: updateRepo releaseService releaseWebsite
