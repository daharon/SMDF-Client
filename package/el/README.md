# Build RPM

## Enterprise Linux 7

```
$ docker build -t smdf-client-el7 ./package/el
$ docker run --rm --volume ${PWD}:/code smdf-client-el7
```
The resulting RPMs will be located in the `target/release/el` directory.

## Repository Config

`/etc/yum.repos.d/smdf.repo`
```
[smdf]
name=smdf
baseurl=https://packagecloud.io/daharon/smdf/el/$releasever/$basearch
repo_gpgcheck=1
gpgcheck=0
enabled=1
gpgkey=https://packagecloud.io/daharon/smdf/gpgkey
sslverify=1
sslcacert=/etc/pki/tls/certs/ca-bundle.crt
metadata_expire=300

[smdf-source]
name=smdf-source
baseurl=https://packagecloud.io/daharon/smdf/el/$releasever/SRPMS
repo_gpgcheck=1
gpgcheck=0
enabled=0
gpgkey=https://packagecloud.io/daharon/smdf/gpgkey
sslverify=1
sslcacert=/etc/pki/tls/certs/ca-bundle.crt
metadata_expire=300
```
