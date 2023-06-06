set -e
cargo build --release
mkdir -p ~/.bin
cp ./target/release/zsh_pinyin_comp ~/.bin/zsh_pinyin_comp
cp ./shell/pinyin-comp.zsh ~/.bin/pinyin-comp.zsh
echo "source ~/.bin/pinyin-comp.zsh" >> ~/.zshrc
echo "export PATH=\$PATH:~/.bin" >> ~/.zshrc
