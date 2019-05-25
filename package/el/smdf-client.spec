Name:           smdf-client
Summary:        SMDF client
Version:        0.1.0
Release:        1%{?dist}
License:        Apache-2.0 with Commons Clause
Group:          Applications/System
Source0:        %{name}.tar.gz
#Source1:        <systemd-service>
Requires:       coreutils openssl
BuildRequires:  rust cargo openssl-devel
BuildRoot:      %{_tmppath}/%{name}-%{version}-%{release}-root

%description
Client for the SMDF monitoring application.

%prep
%setup -c

#%check
#cd %{_builddir}
#cargo test

%build
cargo build --release --target-dir ./target

%install
%{__mkdir} -p %{buildroot}/%{_bindir}
%{__mkdir} -p %{buildroot}/%{_unitdir}
%{__mkdir} -p %{buildroot}/%{_sysconfdir}/sysconfig
%{__cp} ./target/release/%{name} %{buildroot}/%{_bindir}/%{name}
%{__cp} ./package/el/%{name}.service %{buildroot}/%{_unitdir}/%{name}.service
%{__cp} ./package/el/%{name}.sysconfig %{buildroot}/%{_sysconfdir}/sysconfig/%{name}

%clean
rm -rf %{buildroot}

%post
systemctl daemon-reload

%preun
systemctl stop %{name}.service > /dev/null 2>&1
systemctl disable %{name}.service
systemctl daemon-reload

%files
%defattr(-,root,root,-)
%attr(755,root,root) %{_bindir}/%{name}
%attr(644,root,root) %{_unitdir}/%{name}.service
%attr(644,root,root) %config(noreplace) %{_sysconfdir}/sysconfig/%{name}

%changelog
* Fri May 24 2019 Daniel Aharon <dan@danielaharon.com> - 0.1.0-1
- Initial
