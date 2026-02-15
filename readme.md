<h1 align="center">
    Unitt
</h1>

<div align="center">
    <h4>A lean unit-test tool<br>for the 
    <a href="https://github.com/arturo-lang/arturo/">
        Arturo Programming language
    </a>
    </h4>
<a href="https://github.com/RickBarretto/unitt/blob/master/LICENSE" style="text-decoration: none; display: inline-block;"><img src="https://img.shields.io/github/license/RickBarretto/unitt?style=for-the-badge" alt="License"/></a> <a href="https://github.com/arturo-lang/arturo" style="text-decoration: none; display: inline-block;"><img src="https://img.shields.io/badge/language-Arturo-6A156B.svg?style=for-the-badge" alt="Language"/></a> <a href="https://github.com/RickBarretto/unitt/actions" style="text-decoration: none; display: inline-block;"><img src="https://img.shields.io/github/actions/workflow/status/RickBarretto/unitt/test.yml?branch=master&style=for-the-badge" alt="Build Status"/></a>
</div>

----

## At a Glance

<p align="center">
<img 
    alt="Running Unitt from terminal (v3)"
    width="720"
    src="./docs/running unitt - v3.png"
/>
</p>

## Trying Unitt

**Installation**

```
arturo -p install unitt
```

**Setup**

Create a `config.art` file on your project root:

```art
; Configuration for unitt

path: "specs"              ; Path to Tests
suffix: ".spec.art"        ; Test file's suffix
binary: "./bin/arturo"     ; Arturo Binary
```

**Running**

```
unitt
```

### Testing code

A real example of tests:

```art
import {unitt}!

unix?: true

describe "binary appending" [
    it "should operate integers" [
        b: to :binary 0
        expects.be: 'equal? @[express 2 append b 1]
        expects.be: 'equal? @[express 1 b ++ 1]
    ]

    it "should return a binary" [
        b: to :binary 0
        expects.be: 'binary? @[append b 1]
        expects.be: 'binary? @[b ++ 1]
    ]
]

test.skip: unix? "split should deal with windows's paths" [
    expects.be: 'equal? @[
        ["." "splited" "path"]
        split.path ".\\splited\\path"
    ]
]

test "split should deal with unix path" [
    expects.be: 'equal? @[
        ["." "splited" "path"] 
        split.path "./splited/path"
    ]
]

if standalone? ::
    u: unitt!

    u\report 'full
    u\exit
```

This will show you:

```
======== .unitt\example\example.spec.art ========

Describe: Module's Global Specs

    ⏩ split should deal with windows's paths

    ✅ split should deal with unix path
        ✅: equal? [. splited path] [. splited path]

Describe: binary appending

    ❌ - should operate integers
        ❌: equal? "2" 00 01
        ❌: equal? "1" 00 01

    ✅ - should return a binary
        ✅: binary? 00 01
        ✅: binary? 00 01


========== Summary ==========

  ⏏️    TOTAL: 5 assertions
  ✅  PASSED: 3 assertion
  ⏩ SKIPPED: 0 assertions
  ❌  FAILED: 2 assertion

========== ======= ==========

Some tests failed!
```

## Documentation

### *Compatibility*

Unitt supports both: XUnit and RSpec-like syntax:
* The 1st version was heavily influenced by XUnit.
* The 2nd one introduced a RSpec-inspired syntax alternatively, which is now the recommended one.

### *RSpec-ish API*

- `describe: $[description :string tests :block]`:
    Groups tests around some feature.
- `it: $[description :string, testCase :block]`:
    The test case itself, you need to pass a clear description to it,
    And the logic that you're trying to assert.
    - `.skip :logical`:
        Skips tests for some condition. 
        Will just skip if no condition is provided.
- `expects: $[condition :block]`:
    A function that is only available inside the `it`/`test` case,
    makes an assertion given the `condition`.
    - `.to :literal` (or `.be`)
        Uses some function to evaluate the statement.
        This helps to show the function name on display, 
        instead of a `true`/`false`.

### *XUnit-ish API*

- `test: $[description :string, testCase :block]`:
    The same as `it`. 
    - `.skip :logical`
- `assert: $[condition :block]`:
    The same as `expects`
    - `.with`
        The same as `.to` and `.be`
- `suite: $[description :string tests :block]`:
    The same as `describe`.

### `Standalone?` mode

You can also run your tests directly, without using `unitt` CLI.
To ensure this will work for both, `unitt` and `arturo` invokation,
put this under a `if standalone? ::` block.

- `unitt\report: $[mode :literal]`:
    Reports the tests on your terminal.
    - mode:
        - `'full`: Shows all tests and summary
        - `'minimal`: Shows failed tests and summary
- `unitt\exit`:
    Exits with proper code.
- `unitt\result: :dictionary`
    Returns the raw storage. For debugging or additional extensions, related.

## Breaking Changes

There are some breaking changes from the latest major version, so make sure you've some time to change it before update.

1. Script Runner: now, you don't need to setup a script runner anymore. Just delete it. Arturo's package manager has included the feature of script entries for packages, so you can run it such as a common CLI application out-of the box, such like pipx and npx.
2. `test.prop`/`it.prop`: I've added this for property-based tests, but honestly this does absolutely nothing. This attribute just replaces `-` by a `~`. Your test description should be semantic enough to know if this is use-case based or property based. This also reduces a lot the complexity of the library, making it less error-prone.
3. `assert.static`/`expected.static`: This was added when I had no idea how well my evaluator would be. Now, I think this is mature enough to just don't use it at all. Static assertions are not that useful when debugging tests, either. If you need something similar, transform your expression into a `:string`.
4. `fatal`: This was very useful since then, but the way our architecture works now, I think this is kind-of useless. In summary: If some test has an uncaught exception, this stops unitt and shows you the error. For failing tests, you can use the `--clean` flag to show only failing specs.


> [!WARNING]
> Never import this lib as `.lean`, or this will break the current code. This happens due to the nature of Arturo (being concatenative),  and the way we importings are working right now. This may change in future.

---

> Background photo on ["At a Glance"](#at-a-glance) by [Luca Bravo](https://unsplash.com/@lucabravo?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash) on [Unsplash](https://unsplash.com/photos/boat-docked-near-house-VowIFDxogG4?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash)

