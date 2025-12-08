# AHC057

## history

- v0: 一番最初に30個ずつ結合(スコア: 208,521,711, 予測: 208,571,792)

## command

- 1テストケース実行

```
cargo build
cat .\in\0000.txt | .\target\debug\ahc057.exe > .\out\0000.txt
```

- 一括実行

```
cargo build
python .\simulator.py
```

- vis

```
cargo build --release
cat .\in\0000.txt | .\tools\vis.exe .\target\release\ahc057.exe > .\out\0000.txt
```
