#!/usr/bin/env bash

set -e

rpmdev-setuptree

echo '### Creating tarfile for the source RPM'
tar --create \
    --gzip \
    --directory /code \
    --file ~/rpmbuild/SOURCES/smdf-client.tar.gz \
    ./{Cargo.toml,LICENSE,README.md,package/el/smdf-client.{service,sysconfig},src}

echo '### Building RPMs'
rpmbuild -ba \
    --define 'noclean 1' \
    --define '_rpmdir /code/target/release/el' \
    --define '_srcrpmdir /code/target/release/el' \
    /code/package/el/smdf-client.spec
