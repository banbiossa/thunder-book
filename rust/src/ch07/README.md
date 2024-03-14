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

Rust で delegation ぽいことをやりたい場合 portrait (https://crates.io/crates/portrait)

を使って以下のようにやる

```rust
// use_portrait.rs
#[portrait::make]
trait Core {
    fn foo(&self);
    fn bar(&self);
}

struct Data {
    num: usize,
}

impl Core for Data {
    fn foo(&self){
        println!("{}", self.num);
    }
    fn bar(&self){
        println!("i am Data");
    }
}

// Core trait が他モジュールの場合
use crate::<your_mod_tree>::core_portrait;

struct Wraps {
    inner: Data,
}

#[portrait::fill(portrait::delegate(Data; self.inner))]
impl Core for Wraps {
    // bar だけ overwrite する
    fn bar(&self){
        println!("i am Wraps");
    }
}

fn use_core<T: Core>(data: &T){
    data.foo();
    data.bar();
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_use_core(){
        let data = Data {num: 0};
        use_core(&data);
        let wrapped = Wraps {inner: data};
        use_core(&wrapped);
    }
}
```

## やりたいこと

Object Oriented なコードの移植にあたり、どうしても delegation というか、inheritence ぽいことがやりたくなる.

この状態で Wraps は Data に生えてる Core trait を流用したい。

```rust
trait Core {}

struct Data {}
impl Core for Data {}

struct Wraps {
    inner: Data
}
impl Core for Wraps {}
```

## 古い問題

これ自体は古い問題で、2018/04/06 に以下のように議論されている.
RFC: Delegation (https://github.com/rust-lang/rfcs/pull/2393)

> We can see a recurring pattern where the implementation of a method only consists in applying the same method to a subfield or more generally to an expression containing `self`. Those are examples of the well known composition pattern. It has a lot of advantages, but unfortunately requires writing boilerplate code again and again.

## naive な実装

naive に実装すると以下のようになる。これはコンパイルするし意図通りに動くが、先の指摘のように boilerplate コードが増える。

```rust
trait Core {
    fn foo(&self);
    fn bar(&self);
}

struct Data {
    num: usize,
}
impl Core for Data {
    fn foo(&self) {
        println!("num is {}", self.num);
    }
    fn bar(&self){
        println!("i am Data");
    }
}

struct Wraps {
    inner: Data
}
impl Core for Wraps {
    // foo のように inner と同じ挙動をさせる関数が増える場合、
    // それを全て書く必要がある
    fn foo(&self) {
        self.inner().foo();
    }
    // ...

    // bar だけ overwrite する
    fn bar(&self){
        println!("i am Wraps");
    }
}

fn use_core<T: Core>(data: &T){
    data.foo();
    data.bar();
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_use_core(){
        let data = Data {i: 0};
        use_core(&data);
        let wrapped = Wraps {inner: data};
        use_core(&wrapped);
    }
}
```

## Wrap でやろうとする

Claude3 Opus に相談したところ以下のようなコードを示された。やりたいことはわかるし結構賢いが、残念ながらコンパイルしない。

```rust
// この signature が
// associated type bounds are unstable
// となりコンパイルしない
trait Core<T: Core<T: Core>> {
    fn get_inner_state(&self) -> T;
    fn foo(&self) {
        self.get_inner_state().foo();
    }
    fn bar(&self) {
        self.get_inner_state().bar();
    }
}

struct Data {
    num: usize,
}
impl Core<Data> for Data {
    fn get_inner_state(&self) -> Data {
        unimplemented!("this is most inner state");
    }
    fn foo(&self) {
        println!("num is {}", self.num);
    }
    fn bar(&self) {
        println!("i am Data");
    }
}

struct Wraps {
    inner: Data,
}

impl Core<Data> for Wraps {
    fn get_inner_state(&self) -> Data {
        &self.inner
    }
}

fn use_core<T: Core<T: Core>>(data: &T) {
    data.foo();
    data.bar();
}
```

こうなる

```sh
error[E0658]: associated type bounds are unstable
 --> src/ch07/for_blog.rs:1:20
  |
1 | trait Core<T: Core<T: Core>> {
  |                    ^^^^^^^
  |
  = note: see issue 
  = #52662 <https://github.com/rust-lang/rust/issues/52662> 
  = for more information
```


## ambassador を使う

2020年に Qiita 記事があった。 

Rustで委譲をやりたい https://qiita.com/garkimasera/items/8be4a5aa38a7d59d2339

ambassador https://crates.io/crates/ambassador は現在も開発が続いているので良さそう。

以下のように実装する。

```rust
use ambassador::{delegatable_trait, Delegate};

#[delegatable_trait]
trait Core {
    fn foo(&self);
    fn bar(&self);
}

struct Data {
    num: usize,
}

impl Core for Data {
    fn foo(&self) {
        println!("{}", self.num);
    }
    fn bar(&self) {
        println!("i am Data");
    }
}

#[derive(Delegate)]
#[delegate(Core)]
struct Wraps {
    inner: Data,
}

fn use_core<T: Core>(data: &T) {
    data.foo();
    data.bar();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_core() {
        let data = Data { num: 0 };
        use_core(&data);
        let wrapped = Wraps { inner: data };
        use_core(&wrapped);
    }
}


```

しかしこの場合、一部の trait だけ上書きするときにうまくいかない

```rust
impl Core for Wraps {
    fn bar(&self) {
        println!("i am Wraps");
    }
}
```

こうなる

```sh
  --> src/ch07/for_blog.rs:22:10
   |
22 | #[derive(Delegate)]
   |          ^^^^^^^^ conflicting implementation for `Wraps`
...
28 | impl Core for Wraps {
   | ------------------- first implementation here
```

## portrait を使う

StackOverfow で portrait でできるよ、というコメントを見かけたので使う（元の質問はどっかに行ってしまった）。

（冒頭と同じ）
```rust
// delegate させたい trait につける
// core_portrait (<trait_name>_portrait) という名前で　export される
#[portrait::make]
trait Core {
    fn foo(&self);
    fn bar(&self);
}

struct Data {
    num: usize,
}

impl Core for Data {
    fn foo(&self){
        println!("{}", self.num);
    }
    fn bar(&self){
        println!("i am Data");
    }
}

// Core trait が他モジュールの場合
use crate::<your_mod_tree>::core_portrait;

struct Wraps {
    inner: Data,
}

// portrait::fill で delegate 先の object を指定する
// self.inner に指定された Data 型に delegate したいので
// 以下のように書く
#[portrait::fill(portrait::delegate(Data; self.inner))]
impl Core for Wraps {
    // bar だけ overwrite する
    fn bar(&self){
        println!("i am Wraps");
    }
}

fn use_core<T: Core>(data: &T){
    data.foo();
    data.bar();
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_use_core(){
        let data = Data {num: 0};
        use_core(&data);
        let wrapped = Wraps {inner: data};
        use_core(&wrapped);
    }
}
```

## 感想

色々試行錯誤するのも楽しかったし、ちゃんと調べたらいろんな人がハマっていてちゃんと調べてないとダメだなと反省したし、ちゃんとマクロを実装してる crate があって助かった。
