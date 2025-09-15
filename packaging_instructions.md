# Packaging Instructions for Calcutron

## Windows Installer (MSI) - COMPLETED
The Windows installer has been successfully created:
- Location: `target/wix/cutron-0.1.0-x86_64.msi`

## Debian Package (deb) - Requires Linux Environment
To create a Debian package, you would need to run these commands on a Linux system:

```bash
# Install cargo-deb (if not already installed)
cargo install cargo-deb

# Build for Linux target
rustup target add x86_64-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu --release

# Create the Debian package
cargo deb --target x86_64-unknown-linux-gnu
```

The resulting .deb file would be located in `target/debian/`.

## RPM Package (rpm) - Requires Linux Environment
To create an RPM package, you would need to run these commands on a Linux system with RPM build tools:

```bash
# Install cargo-rpm
cargo install cargo-rpm

# Initialize RPM packaging (first time only)
cargo rpm init

# Build and package
cargo rpm build
```

The resulting .rpm file would be located in `target/release/rpmbuild/RPMS/x86_64/`.

## Cross-platform Building (Alternative)
If you want to create all packages from a Windows machine, you could use Docker containers or a CI/CD pipeline:

1. Use Docker with Linux images to build and package for Linux targets
2. Use GitHub Actions or similar CI/CD services to build packages for different platforms