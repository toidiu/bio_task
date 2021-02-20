.DEFAULT_GOAL := all

# biopony here
APACHE_DIR=/var/www
BIO_DIR=biotask

all: releaseService releaseWebsite

updateRepo:
	git pull

releaseService:
	pkill "bio_task"; \
	cd service && cargo build --release && \
	(./target/release/bio_task &)

releaseWebsite:
	cd website && \
	npm install && \
	npm run build && \
	mkdir -p ${APACHE_DIR}/${BIO_DIR} && \
	mv dist/* ${APACHE_DIR}/${BIO_DIR}

.PHONY: webBuildAndRelease





.PHONY: updateRepo releaseService releaseWebsite
