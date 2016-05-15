# How to Contribute

Contributions are always welcome!
Please use the following guidelines when contributing to `colonize`:

1. Fork `colonize`
2. Clone your fork (`git clone https://github.com/$YOUR_USERNAME/colonize && cd colonize`)
3. Create a new branch (`git checkout -b new-branch`)
4. Make your changes
5. Commit your changes (`git commit -am "your message"`)
 * This project uses the
 [conventional changelog format][conventional-changelog-format], so that we can
 easily update the `CHANGELOG.md` using [clog][clog-cli].
 * In addition to the conventions defined above, we also use `imp`, `wip`,
   and `examples`.
 * Format your commit subject line using the following format:
   `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `feat` - A new feature
    - `imp` - An improvement to an existing feature
    - `perf` - A performance improvement
    - `docs` - Changes to documentation only
    - `tests` - Changes to the testing framework or tests only
    - `fix` - A bug fix
    - `refactor` - Code functionality doesn't change, but underlying structure
      may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work in progress commit (Should typically be `git rebase`'ed
      away)
    - `chore` - Catch all or things that have to do with the build system, etc.
    - `examples` - Changes to an existing example, or a new example
 * The `COMPONENT` is optional, and may be a single file, directory, or logical
   component.
   Can be omitted if commit applies globally
6. Run the tests (`cargo test`)
7. `git rebase` into concise commits and remove `--fixup`s
   (`git rebase -i HEAD~NUM` where `NUM` is number of commits back)
8. Push your changes back to your fork (`git push origin $your-branch`)
9. Create a pull request! (You can also create the pull request first, and
   we'll merge when ready.
   This a good way to discuss proposed changes.)

[clog-cli]: https://github.com/clog-tool/clog-cli "clog-tool/clog-cli"
[conventional-changelog-format]: https://github.com/angular/angular.js/blob/master/CONTRIBUTING.md#commit "Angular Git Commit Guidelines"
