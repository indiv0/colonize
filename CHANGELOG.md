<a name="v0.4.0"></a>
## v0.4.0 (2020-12-15)


#### Improvements

*   Move camera closer to dwarf landing zone ([674d6bc4](https://github.com/indiv0/colonize/commit/674d6bc4b710ab910f4078db08a0503ca668283d))

#### Features

*   Add rigidbody physics to chunks and dwarves ([0c27dcbd](https://github.com/indiv0/colonize/commit/0c27dcbd9459261d42c27fa04a78353304d2ed2e))
*   Add bevy_rapier3d for basic physics ([c5f7ca9a](https://github.com/indiv0/colonize/commit/c5f7ca9a9a5d7ffe014d2fa677eb79fe45666964))



<a name="v0.3.0"></a>
## v0.3.0 (2020-12-13)


#### Features

*   Add placeholder Dwarf plugin ([3bba076c](https://github.com/indiv0/colonize/commit/3bba076ca50b73a649d8a5dc240d7c3a62dece1c))
*   Add noise-based terrain ([0218fa33](https://github.com/indiv0/colonize/commit/0218fa33facb171c7cb72bc66398b5bed8c2fdec))
*   Add some basic building-blocks meshing with upstream snappy changes ([d90f3b53](https://github.com/indiv0/colonize/commit/d90f3b538180a8dc984bc92b4ad7b6cfe6e7e204))
*   Add initial usage of building-blocks ([8a211a12](https://github.com/indiv0/colonize/commit/8a211a12760e6d71baba4a542ab909dc91cc0785))
*   Add tesselation library for dual contouring ([62c63b11](https://github.com/indiv0/colonize/commit/62c63b1103ef5aa9ff00cb0b0e6ff787424336a6))
*   Add broken pan-orbit (aka arcball) camera implementation ([b8b5aa2f](https://github.com/indiv0/colonize/commit/b8b5aa2f8187e0925be56412531c789b4d3978e5))
*   Add toggle for cursor visibility and locking ([f5c8a151](https://github.com/indiv0/colonize/commit/f5c8a151fad24049650059291402608dfea97842))
*   Add mesh picking support via bevy_mod_picking ([0dc2d16e](https://github.com/indiv0/colonize/commit/0dc2d16e06264c017f271b68d01d7985546e910b))
*   Add basic camera plugin which allows for motion in 3D space ([fdaa86cf](https://github.com/indiv0/colonize/commit/fdaa86cffd326955695ac5f6ea41923938314b71))
*   Add support for compiling to WASM ([e87a0efd](https://github.com/indiv0/colonize/commit/e87a0efd3a1176aadee7cefb01cdebda2417812d))
*   Add basic 3D scene ([599c2509](https://github.com/indiv0/colonize/commit/599c250942d2be28eb5cee39c257c53743d98ecb))
*   Add initial Bevy hello world example ([16eae019](https://github.com/indiv0/colonize/commit/16eae0197f7b825d26b6c8b297213350aa697fc6))

#### Improvements

*   Spawn some "dwarf" cubes ([45daf208](https://github.com/indiv0/colonize/commit/45daf20845cf62ec928edc4cb7c8a08e1b9ea64a))
*   Fix rustc and clippy lints ([28858e47](https://github.com/indiv0/colonize/commit/28858e4756f8b440aaff56418c08a1b1f4975d2a))
*   Switch to local building-blocks fork and randomized terrain ([47a8d3b6](https://github.com/indiv0/colonize/commit/47a8d3b623cae9822f86da5b29005a9f01901513))
*   Add support for tilting with numpad keys ([9f94918b](https://github.com/indiv0/colonize/commit/9f94918b607ca095bae456226467fd5376b409fe))
*   Update for latest bevy commit & enable debug optimizations ([78f72990](https://github.com/indiv0/colonize/commit/78f729903ee6233addecbdcf8b8712c8d40de53d))

#### Documentation

*   Add link to guide on enabling fast incremental compiles ([ba58c27c](https://github.com/indiv0/colonize/commit/ba58c27c923143ee1f47fe27c2d75aa13a885be0))



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
