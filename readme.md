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
    alt="Running Unitt from terminal"
    width="720"
    src="./docs/running unitt screenshot.png"
/>
</p>

## Trying Unitt

**Installation**

```
arturo -p install unitt
```

**Execution**

```sh
# Runs test/test*.art by default
unitt 
```

```sh
# Glob Pattern from Shell
unitt test/*test.art
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

> [!NOTE]
> Property-based tests have `~` as separator. 

### The *Runner*


```
===== Statistics =====

⏏️   TOTAL: 24 assertions
✅  PASSED: 20 assertions
⏩ SKIPPED: 4 assertions
❌  FAILED: 4 assertions

===== ========== =====
```

Also, the runner is able to return an error code, 
so that is great if you're working with *Continuous Integration*.

## Documentation

### *Unitt*
- `test: $[description :string, testCase :block]`:
    The test case itself, you need to pass a clear description to it,
    And the logic that you're trying to assert.
    - `.prop`:
        Indicates that a test is property-based.
    - `.skip :logical`:
        Skips tests for some condition. 
        If none condition is given, this will just skip the test.
    - `.static: :block`:
        Defines what will and what won't be evaluated.
    - `.static: :logical`:
        Disable runtime evaluation, and forces static display.
- `assert: $[condition :block]`:
    A function that is only available inside the `test` case,
    makes an assertion given the `condition`.
- `suite: $[description :string tests :block]`:
    Visually groups tests together.

## *Compatibility*

This section includes the old-syntax inspired by XUnit. 
Kept for compatibilities with our 1st version.

- `test: $[description :string, testCase :block]`:
    The same as `it`. 
    Not only kept for compatibility issues,
    but great to be used when not into a `describe`/`suite` block.
    - `.prop`: (Temporarially deprecated)
        Indicates that a test is property-based.
    - `.skip :logical`:
        Skips tests for some condition. 
        If none condition is given, this will just skip the test.
    - `.static: :logical`:
        Disable runtime evaluation, and forces static display.
- `assert: $[condition :block]`:
    The same as `expects`
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
  by [Jack Anstey](https://unsplash.com/@jack_anstey?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash) on [Unsplash](https://unsplash.com/photos/aerial-photography-of-road-zS4lUqLEiNA?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash)