<a name="v0.1.0"></a>
## v0.1.0 (2016-05-16)


#### Bug Fixes

*   fix struct deserialization ([eb38933a](https://github.com/indiv0/colonize/commit/eb38933ad4703dc470f006632de0a17e7a39955a))
*   fix compilation on stable ([4d135c35](https://github.com/indiv0/colonize/commit/4d135c359f9518d072d8b8081a5aa6b75331aaee))
* **Camera:**  fix initial `Camera` position ([abbf5643](https://github.com/indiv0/colonize/commit/abbf56431b5d44672a6d81307cea90ac9115c47f), closes [#82](https://github.com/indiv0/colonize/issues/82))

#### Features

*   display render debug info in window ([1a590c1e](https://github.com/indiv0/colonize/commit/1a590c1e5411c75666461e4d7f9ff1a952abe2f2))
*   implement partially backend-agnostic rendering ([5acfcd49](https://github.com/indiv0/colonize/commit/5acfcd4933e765f85d7da5cae71c2ffcd612c071), closes [#22](https://github.com/indiv0/colonize/issues/22), breaks [#](https://github.com/indiv0/colonize/issues/))
*   move `tcod` crate import to `worldview` ([659f41a8](https://github.com/indiv0/colonize/commit/659f41a85695b1a1ac81e4b50bf42b5edd518c27))
*   improve terrain generation ([919840dd](https://github.com/indiv0/colonize/commit/919840dd2cc5880cba27bdb4ed1ff656c61a81ef))
*   refactor rendering code from `world` ([f1ddf577](https://github.com/indiv0/colonize/commit/f1ddf577a7eb77cd5aa1cf3a214e3735dab13506))
*   implement 3D chunk generation ([d38b4a45](https://github.com/indiv0/colonize/commit/d38b4a45dba632d227abd8f20a4230dd29e9a6e8))
*   refactor out `Direction` enum ([e866c025](https://github.com/indiv0/colonize/commit/e866c0255aa8a2101cd077f8e1bfedd8845db4d6))
*   replace `Point2` with `cgmath::Point2` ([1b3313c9](https://github.com/indiv0/colonize/commit/1b3313c9ddc8d8c59a0c2ce86c0303f12e46b647))
*   add `Camera` struct and `Command` trait ([54712b69](https://github.com/indiv0/colonize/commit/54712b6918caf1aa1c0caf05e2742ef54c7f4a19))
*   update to latest Rustc ([c6409450](https://github.com/indiv0/colonize/commit/c64094508343f6a59a2dc13d43a8b6bf1186ad69))
* **Localization:**  remove unnecessary pub ([7036912b](https://github.com/indiv0/colonize/commit/7036912b1c416ea51020105c68581ee31dccd07f))
* **macro:**  remove unused macro arg ([e5224b80](https://github.com/indiv0/colonize/commit/e5224b80ff6da35873d6a949db69fa102ab796af))

#### Breaking Changes

*   implement partially backend-agnostic rendering ([5acfcd49](https://github.com/indiv0/colonize/commit/5acfcd4933e765f85d7da5cae71c2ffcd612c071), closes [#22](https://github.com/indiv0/colonize/issues/22), breaks [#](https://github.com/indiv0/colonize/issues/))

#### Documentation

* **CONTRIBUTING.md:**  add `CONTRIBUTING.md` ([311f79ac](https://github.com/indiv0/colonize/commit/311f79ac80fccb9b4891e761e689d9cdec6ac084), closes [#72](https://github.com/indiv0/colonize/issues/72))
* **CONTRIBUTORS.md:**  add `CONTRIBUTORS.md` ([0b5cd45c](https://github.com/indiv0/colonize/commit/0b5cd45cc824edfe5bf6d635dc123cc39b2fd6c6), closes [#73](https://github.com/indiv0/colonize/issues/73))
* **Cargo.toml:**  update `Cargo.toml` files ([8e6dcdb9](https://github.com/indiv0/colonize/commit/8e6dcdb91bdf32ff7d750d9d361cc6e7d9d543e1), closes [#76](https://github.com/indiv0/colonize/issues/76))
* **LICENSE:**  replace LICENSE ([f5d21499](https://github.com/indiv0/colonize/commit/f5d21499743376c13a6790ef3e2fec6e2e1e65c3), closes [#74](https://github.com/indiv0/colonize/issues/74))
* **README:**
  *  replace out-dated `Config` info ([b7bb4c5b](https://github.com/indiv0/colonize/commit/b7bb4c5b05f5b5e97eced4bebca26e1939a60cbb), closes [#83](https://github.com/indiv0/colonize/issues/83))
  *  add platforms & toolchains info ([956272f3](https://github.com/indiv0/colonize/commit/956272f3e1eb984e7170f872677ecba2dc4a65a5), closes [#75](https://github.com/indiv0/colonize/issues/75))
  *  replace Crates.io badges ([7925364c](https://github.com/indiv0/colonize/commit/7925364c63f9a4d06e18f3054702027cd12e071d))
  *  add in-game screenshot ([19c5c7f0](https://github.com/indiv0/colonize/commit/19c5c7f0481aa861d67119d47a4452027ddc961d), closes [#61](https://github.com/indiv0/colonize/issues/61))
  *  add CHANGELOG notice ([972d7f5b](https://github.com/indiv0/colonize/commit/972d7f5b402614309e3119681fa736c4226bdd05))
  *  update `README.md` ([07cd376b](https://github.com/indiv0/colonize/commit/07cd376b496ac9c3299426e238c4ba6117d26284))
