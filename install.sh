cargo build --release
cp ./target/release/zsh_pinyin_comp ~/.bin/zsh_pinyin_comp
mkdir -p ~/.bin
cp ./shell/pinyin-comp.zsh ~/.bin/pinyin-comp.zsh
echo "source ~/.bin/pinyin-comp.zsh" >> ~/.zshrc
