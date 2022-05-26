## 注意

Rust 学習中の練習用リポジトリ

学習のために作成してるコードになりますので、クローンして使用する際は自己責任でお願い致します。

## 説明

コマンドライン引数からファイルパスを1つだけ読み込んで
そのファイルが `Shift_JIS` である前提で(改行コードは切り捨てられる)
utf8 に変換し、改行コード LF (というか、プラットフォームの改行コード)で標準出力へ送る

## 参考

- [`[Rust] Shift_JISをUTF-8に変換する`](https://qiita.com/V_lasergun/items/4926654ab4bd1ebc3d6c)
- [`Rustで高速な標準出力`](https://keens.github.io/blog/2017/10/05/rustdekousokunahyoujunshutsuryoku/)

