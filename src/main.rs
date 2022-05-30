// 参考 : https://qiita.com/V_lasergun/items/4926654ab4bd1ebc3d6c
//        https://keens.github.io/blog/2017/10/05/rustdekousokunahyoujunshutsuryoku/
use encoding_rs; // Shift_JIS のファイルからデコードするに必要
use std::env;
use std::fs;
use std::io::{stdout, Write, BufWriter};

type MyResult = Result<(), Box<dyn std::error::Error>>;

// コマンドライン引数からファイルパスを1つだけ読み込んで
// そのファイルが Shift_JIS である前提で(改行コードは切り捨てられる)
// utf8 に変換し、改行コード LF (というか、プラットフォームの改行コード)で標準出力へ送る
fn main() -> MyResult {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // ここは参照である必要がある。
    let out = stdout();
    // lock して排他処理を回避(スピードアップ目的)
    let mut out = BufWriter::new(out.lock());
    let b = fs::read(filename)?;
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&b);
    let text = res.into_owned();
    // 行に分割してループする段階で元の改行コードを破棄
    for line in text.lines() {
        // writeln! を使用することでプラットフォームの改行コード＝LFで出力
        writeln!(out, "{}", line)?; 
    }
    out.flush().unwrap();
    Ok(())
}
