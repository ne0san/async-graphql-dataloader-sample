# async-graphql-dataloader-sample

Async-graphql での Dataloader 実装による N+1 問題の解決例

## 構築まで

1. `sea-orm-cli`をインスト

```
cargo install sea-orm-cli
```

2. `.env`を作成

```env
DATABASE_URL=postgres://user:pass@localhost/dbname
↓
DATABASE_URL=mysql://root:root@localhost:3306/mytest
```

3. migrate init を実行

```
sea-orm-cli migrate init
```

### マイグレーション作成 entity を使う場合

1. `cargo new entity --lib`
2. entity の cargo.toml に sea-orm を足す

```
sea-orm = { version = "0.12.15", features = [ "sqlx-mysql", "runtime-async-std-native-tls", "macros" ] }
```

3. ソースを修正して entity を定義する

4. migration のクレートから entity クレートが参照できるように dependances を追加

```toml
[dependencies]
entity = { path = "../entity" }
```

5. migration の作成

6. migration を作成したら lib.rs にも追加

7. migration の実行

```
sea-orm-cli migrate up
```

## n+1 が起きる時のクエリ

ユーザ 3 つ
ユーザ全取得クエリと、それぞれのユーザに対する item 取得で計 4 本クエリが走っている

```graphql
query {
  users {
    id
    description
    items {
      id
      description
    }
  }
}
```

```log
2024-07-21T03:42:47.410413Z DEBUG sea_orm::driver::sqlx_mysql: SELECT `user`.`id`, `user`.`name`, `user`.`description`, `user`.`deleted`, `user`.`created_at`, `user`.`updated_at` FROM `user`
2024-07-21T03:42:47.423393Z DEBUG sqlx::query: summary="SELECT `user`.`id`, `user`.`name`, `user`.`description`, …" db.statement="\n\nSELECT\n  `user`.`id`,\n  `user`.`name`,\n  `user`.`description`,\n  `user`.`deleted`,\n  `user`.`created_at`,\n  `user`.`updated_at`\nFROM\n  `user`\n" rows_affected=0 rows_returned=3 elapsed=4.643542ms elapsed_secs=0.004643542
2024-07-21T03:42:47.424350Z DEBUG sea_orm::driver::sqlx_mysql: SELECT `item`.`id`, `item`.`name`, `item`.`user_id`, `item`.`description`, `item`.`deleted`, `item`.`created_at`, `item`.`updated_at` FROM `item` INNER JOIN `user` ON `user`.`id` = `item`.`user_id` WHERE `user`.`id` = 1
2024-07-21T03:42:47.425453Z DEBUG sea_orm::driver::sqlx_mysql: SELECT `item`.`id`, `item`.`name`, `item`.`user_id`, `item`.`description`, `item`.`deleted`, `item`.`created_at`, `item`.`updated_at` FROM `item` INNER JOIN `user` ON `user`.`id` = `item`.`user_id` WHERE `user`.`id` = 2
2024-07-21T03:42:47.426125Z DEBUG sea_orm::driver::sqlx_mysql: SELECT `item`.`id`, `item`.`name`, `item`.`user_id`, `item`.`description`, `item`.`deleted`, `item`.`created_at`, `item`.`updated_at` FROM `item` INNER JOIN `user` ON `user`.`id` = `item`.`user_id` WHERE `user`.`id` = 3
2024-07-21T03:42:47.438045Z DEBUG sqlx::query: summary="SELECT `item`.`id`, `item`.`name`, `item`.`user_id`, …" db.statement="\n\nSELECT\n  `item`.`id`,\n  `item`.`name`,\n  `item`.`user_id`,\n  `item`.`description`,\n  `item`.`deleted`,\n  `item`.`created_at`,\n  `item`.`updated_at`\nFROM\n  `item`\n  INNER JOIN `user` ON `user`.`id` = `item`.`user_id`\nWHERE\n  `user`.`id` = ?\n" rows_affected=0 rows_returned=30 elapsed=6.392875ms elapsed_secs=0.006392875
2024-07-21T03:42:47.445255Z DEBUG sqlx::query: summary="SELECT `item`.`id`, `item`.`name`, `item`.`user_id`, …" db.statement="\n\nSELECT\n  `item`.`id`,\n  `item`.`name`,\n  `item`.`user_id`,\n  `item`.`description`,\n  `item`.`deleted`,\n  `item`.`created_at`,\n  `item`.`updated_at`\nFROM\n  `item`\n  INNER JOIN `user` ON `user`.`id` = `item`.`user_id`\nWHERE\n  `user`.`id` = ?\n" rows_affected=0 rows_returned=20 elapsed=13.538042ms elapsed_secs=0.013538042
2024-07-21T03:42:47.451147Z DEBUG sqlx::query: summary="SELECT `item`.`id`, `item`.`name`, `item`.`user_id`, …" db.statement="\n\nSELECT\n  `item`.`id`,\n  `item`.`name`,\n  `item`.`user_id`,\n  `item`.`description`,\n  `item`.`deleted`,\n  `item`.`created_at`,\n  `item`.`updated_at`\nFROM\n  `item`\n  INNER JOIN `user` ON `user`.`id` = `item`.`user_id`\nWHERE\n  `user`.`id` = ?\n" rows_affected=0 rows_returned=10 elapsed=18.348166ms elapsed_secs=0.018348166
```

# 参考文献

[Rust で DB マイグレーションの仕組みを導入(SeaORM)](https://qiita.com/kawajit/items/7ebb7ab4e221b27ee847#%E7%92%B0%E5%A2%83)

# tips

rust-analyzer がマクロ展開できない的なエラー吐いたら
->rustup update などでなるかも
