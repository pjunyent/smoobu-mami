# Generated by rust2rpm 25
%bcond_without check

# prevent library files from being installed
%global __cargo_is_lib() 0

%global crate smoobu-mami

Name:           smoobu-mami
Version:        0.1.0
Release:        %autorelease
Summary:        # FIXME

SourceLicense:  EUPL-1.2
# FIXME: paste output of %%cargo_license_summary here
License:        # FIXME
# LICENSE.dependencies contains a full license breakdown

URL:            # FIXME
Source:         # FIXME

BuildRequires:  cargo-rpm-macros >= 24

%global _description %{expand:
%{summary}.}

%description %{_description}

%prep
%autosetup -n %{crate}-%{version} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%files
%license LICENSE
%license LICENSE.dependencies
%{_bindir}/smoobu-mami

%changelog
%autochangelog
