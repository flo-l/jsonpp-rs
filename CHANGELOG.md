Changelog
=================

### 2.0

- Rename the project to `jsonpp`.
- Internal code cleanup (rustfmt)

### 1.1

- fix seemingly random ordering of fields of prettified json
- allow files that don't end with `.json`
- internal improvements, which should make memory usage constant and thus improve performance
- make input parameter optional, read from stdin if not given
- add some hooks to customize output, I'll happily accept PRs adding more hooks

### 1.0

Initial release
