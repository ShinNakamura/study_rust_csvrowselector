# CSV の条件に合うレコードを抜き出す

## 注意

Rust 勉強中の自習課題です。

クローン、使用は自己責任でお願い致します。

## 仕様

まずは素朴な実装から考える。

- コマンドライン引数は全部条件指定に使ってしまう。
- 入力は標準入力からのみ
- 出力は標準出力のみ

sample : tests/members.csv
```csv
id,name,age
10-1,John,30
10-2,Ken,33
15-0,Bob,18
n-09,Jen,48
```

引数にてカンマ区切りで左辺がフィールド名、右辺が値(完全一致)を指定できる
```sh
<tests/members.csv cargo run -- id,10-1 id,10-2 >tests/expect-2ids.csv
```
このコマンドラインの出力は下記のようになる。

sample : tests/expect-2ids.csv
```csv
id,name,age
10-1,John,30
10-2,Ken,33
```

つまり条件指定が複数あった場合、or で結合して判定される。

## 背景的コメント

CSV 内の ID 的フィールドに対してほしい ID を複数指定してその分だけのレコードに絞られた CSV が手に入ればまずはよしと考える。
