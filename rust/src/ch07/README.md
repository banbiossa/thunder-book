# blog: Rust で delegation したいときの portrait

TOC

- 結論
- やりたいこと
- 古い問題
- naive な実装
- Wrap でやろうとする
- ambassador でやる
- portrait でやる


## 結論

Rust で delegation ぽいことをやりたい場合は portrait を使って以下のようにやる

```rust

```

## やりたいこと

Object Oriented なコードの移植にあたり、どうしても delegation というか、inheritence ぽいことがやりたくなる

```rust
```

## 古い問題

これ自体は古い問題で、2018/04/06 に以下のように議論されている. [RFC: Delegation](https://github.com/rust-lang/rfcs/pull/2393).


> We can see a recurring pattern where the implementation of a method only consists in applying the same method to a subfield or more generally to an expression containing `self`. Those are examples of the well known composition pattern. It has a lot of advantages, but unfortunately requires writing boilerplate code again and again.

## naive な実装

naive に実装すると以下のようになり、これはコンパイルするし意図通りに動くか先の指摘のように boilerplate コードが増える。

## Wrap でやろうとする

Claude3 Opus に相談したところ以下のようなコードを示された。やりたいことはわかるし結構賢いが、残念ながらコンパイルしない。

## ambassador を使う

2020年に Qiita 記事があった。 [Rustで委譲をやりたい](https://qiita.com/garkimasera/items/8be4a5aa38a7d59d2339).

[ambassador](https://crates.io/crates/ambassador)は現在も開発が続いているので良さそう。

以下のように実装する。

```
```

しかしこの場合、一部の trait だけ上書きするときにうまくいかない

```
```

## portrait を使う

StackOverfow で portrait でできるよ、というコメントを見かけたので使う（元の質問はどっかに行ってしまった）。

```
```
