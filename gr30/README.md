# gr30


- 問題ごとに `src/bin/A.rs` のように実装します。
- サンプルは `samples/a.in` / `samples/a.out` にコピペします（`.out` は任意）。


## よく使うコマンド
```bash
just run A # samples/a.in で実行
just sample-test A # .out があれば比較
just pack A # dist/A.rs に単一ファイルを生成（提出はこれをコピペ）