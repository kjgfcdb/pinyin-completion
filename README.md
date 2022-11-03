# zsh-pinyin-completion

Complete path by acronym of pinyin initials.

用拼音补全命令行中的中文名称和路径

This is a fork for enhancement and bugs fixing.

### Requirements

- Rust
- Zsh

### Enhancement

Written with rust and support fuzzy pinyin completion.

### Sample

    $ ls .
    我的相册 中ying夹杂.txt
    $ cd w        <tab>             补全[我的相册]
    $ cd wdxc     <tab>             补全[我的相册]
    $ cd xiangce  <tab>             补全[我的相册]
    $ vim yingjia <tab>             补全[中ying夹杂.txt]

### Install

    sh install.sh
