## update and install some things we should probably have
apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  vim \
  build-essential \
  openssl \
  unzip \
  cmake

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y
rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy
rustup component add clippy --toolchain nightly
source "$HOME/.cargo/env"

cargo install cargo-expand
cargo install cargo-edit
cargo install cargo-deny
cargo install cargo-about
cargo install cargo-insta

## setup and install oh-my-zsh

sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"

## Install protoc
PROTOC_VERSION=25.0
curl -L "https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOC_VERSION/protoc-$PROTOC_VERSION-linux-x86_64.zip" --output protoc.zip
unzip protoc.zip -d "$HOME/.local"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
chsh -s $(which zsh)
