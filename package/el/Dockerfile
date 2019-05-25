FROM centos:7

VOLUME ['/code']
ADD build.sh /usr/local/bin
RUN chmod 755 /usr/local/bin/build.sh

# EPEL & RPM build tools
RUN yum --assumeyes install epel-release
RUN yum clean all
RUN yum --assumeyes install rpm-build rpmdevtools

# SMDF Client dependencies.
RUN yum --assumeyes install rust cargo openssl-devel

RUN useradd build
USER build

ENTRYPOINT /usr/local/bin/build.sh
