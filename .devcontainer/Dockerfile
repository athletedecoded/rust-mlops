FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld lshw jupyter-notebook python3-venv python3-dev python3-pip python3-setuptools python3-wheel libssl-dev \
   && apt-get -y install pkg-config libfreetype6-dev libfontconfig1-dev\
   && apt-get -y install sqlite3 libsqlite3-dev \
   && apt-get autoremove -y && apt-get clean -y
