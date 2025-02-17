<h1 align="center">
    Unitt
</h1>

<p align="center">
    <i>Unitt</i> is a lean unit-test tool for the 
    <a href="https://github.com/arturo-lang/arturo/">
        Arturo Programming language
    </a>
    <br><br>
    <img 
        alt="Arturo logo" 
        width="20" 
        src="https://github.com/arturo-lang/arturo/raw/master/docs/images/logo.png#gh-light-mode-only"
    />
    <img 
        alt="Arturo logo" 
        width="20" 
        src="https://github.com/arturo-lang/arturo/raw/master/docs/images/logo-lightgray.png#gh-dark-mode-only" 
    />
</p>

## At a Glance

<p align="center">
<img 
    alt="Running Unitt from terminal (v2)"
    width="720"
    src="./docs/running unitt screenshot.png"
/>
</p>

## Trying Unitt

**Installation**

```
arturo -p install unitt
```

**Setup & Execution**

Create the file `test` on the root of your project:

```art
#! arturo

import {unitt}!

tryOr: $[action :block alt :any][
    (throws? [val:] ++ action)? -> alt -> val
]

files: switch empty? args\values
    -> findTests "tests"
    -> args\values

runTests
    .fatal: tryOr [args\fatal] false
    .suppress: tryOr [args\suppress] false
    files
```

```sh
# Runs test/test*.art by default
./test
```

```sh
# Glob Pattern from Shell
./test test/*test.art
```

### Testing code

A real example of tests:

```art
import {unitt}!

unix?: true

describe "binary appending" [
    it "should operate integers" [
        b: to :binary 0
        expects.be: 'equal? @[as.binary 2 append b 1]
        expects.be: 'equal? @[as.binary 1 b ++ 1]
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
```

This will show you:

```
===== example.art =====

Suite: binary appending 

    ❌ - assert that should operate integers
         ❌: equal? 10 00 01
         ❌: equal? 1 00 01

    ✅ - assert that should return a binary
         ✅: binary? 00 01
         ✅: binary? 00 01


⏩ - assert that split should deal with windows's paths
     skipped!

✅ - assert that split should deal with unix path
     ✅: equal? ["." "splited" "path"] ["." "splited" "path"]


===== Statistics =====

 ⏏️   TOTAL: 3 assertions
✅  PASSED: 2 assertions
⏩ SKIPPED: 1 assertions
❌  FAILED: 1 assertions

===== ========== =====
```

## Documentation

### *Unitt*
- `describe: $[description :string tests :block]`:
    Groups tests around some feature.
- `it: $[description :string, testCase :block]`:
    The test case itself, you need to pass a clear description to it,
    And the logic that you're trying to assert.
    - `.prop`:
        Indicates that a test is property-based.
        The indicator is the `~` separator on the description.
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
    - `.static :logical`:
        Shows it as static code.


## *Compatibility*

This section includes the old-syntax inspired by XUnit. 
Kept for compatibilities with our 1st version.

- `test: $[description :string, testCase :block]`:
    The same as `it`. 
    Not only kept for compatibility issues,
    but great to be used when not into a `describe`/`suite` block.
    - `.prop`
    - `.skip :logical`
- `assert: $[condition :block]`:
    The same as `expects`
    - `.with`
        The same as `.to` and `.be`
     - `.static: :logical`
- `suite: $[description :string tests :block]`:
    The same as `describe`.

### *Setup*
- `runTests: $[tests [:string]]`:
    The *runner function*, this executes all `tests`,
    show statistics and return a value. 
    - `.fatal`:
        Fails on the first error found (per file).
    - `.suppress`: 
        Always return 0 as error code. 
- `findTests: $[folder :string]`:
    Looks for *tests* inside `folder`.
    The default *test* pattern is "test*.art".
    - `.thatMatches :string`:
        Defines what is a test-file via a kind-of *glob* pattern.
        Use a `*` as spliter. 
        - Obs.: That is a kind-of *glob* pattern, not a real one. 
          So just use one and only one `*` to split the pre and suffix.


> [!WARNING]
> Never import this lib as `.lean`, or this will break the current code.
> This happens due to the nature of Arturo (being concatenative), 
> and the way we importings are working right now.
> This may change in future.

---

> Background photo on ["At a Glance"](#at-a-glance) 
by [Artem Sapegin](https://unsplash.com/@sapegin?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash) 
on [Unsplash](https://unsplash.com/photos/brown-wooden-boat-floating-on-body-of-water-XGDBdSQ70O0?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash)
      
