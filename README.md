# Hello, WebAssembly!

Rust で書いた関数を JavaScript から呼び出すテスト

## 使い方

macOS Mojave で作業した。まずは Xcode をインストールする。

- [Xcode - インストール可能 対応バージョン macOS 一覧 / Install Support macOS Version Lists 【 2020.06 】 - Qiita](https://qiita.com/thinkalot/items/1dfdba642906c1bf1fd2)

これによると Mojave に対応しているのは Xcode 11.3.1 までなので Apple Developer からダウンロードする。

- [More Software Downloads - Apple Developer](https://developer.apple.com/download/more/?name=Xcode)

ダウンロードした .xip ファイルを展開すると Xcode.app ができるので、 `/Applications` に移動する。

Rust をアップデート

```shell-session
$ rustup update
...

$ rustc --version
rustc 1.52.1 (9bc8c42bb 2021-05-09)
```

## プロジェクトを新規に作るときは……（Rust 初心者向け）

次のコマンドで wasm-hello というディレクトリができ、その中に Cargo.toml と src/lib.rs ができる。

```shell-session
$ cargo new --lib wasm-hello
```

## 参考

- [Rust から WebAssembly にコンパイルする - WebAssembly | MDN](https://developer.mozilla.org/ja/docs/WebAssembly/Rust_to_wasm)
- [RustではじめるWebAssemblyはじめのいっぽ、足し算するwasmファイルは116byte #rust #wasm #js / 福野泰介の一日一創 / Create every day by Taisuke Fukuno](https://fukuno.jig.jp/2918)
