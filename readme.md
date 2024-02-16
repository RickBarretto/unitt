<h1 align="center">
    Unitt
</h1>

<p align="center">
    <i>Unitt</i> is a basic unit-test tool for the 
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

*Unitt* may be splited into two sections: *runner* and *the tests*.

*The runner* is the section responsible to find, run and return error codes to the final user.
While *the tests* are responsible to group the rules and logic of the tests.

### Initial setup

It's recomended that your *runner* be at the root of your directory, right before your *tests* folder.

All of your *tests* must begin with the `test` prefix and end with the `.art` extension to be found,
since you may want to mix them with some other files.

Being that said, that is the right way of setting up your *runner*:

Let's consider that you have the following directory:

```
src/
    ...
tests/
    ...
main.art
tester.art
```

Into your `tester.art`, you must:

```art
import {unitt}!

runTests "tests"
```

To run it, call:

```
arturo tester.art
```

> [!TIP] 
> You may want to use a hashbang to don't need to call arturo for every run.

### The *tests* itself

A real example of tests:

```art
unix?: true

test "appending binaries with integer is working" [
    b: to :binary 0
    assert -> as.binary 2 = append b 1
    assert -> as.binary 1 = b ++ 1
]

test.prop "appending binaries with integer returns a binary" [
    b: to :binary 0
    assert -> binary? append b 1
    assert -> binary? b ++ 1
]

test.skip: unix? "split is working for windows's paths" [
    assert -> ["." "splited" "path"] = split.path ".\\splited\\path"
]
```

This will show you:

```
❌ - assert that appending binaries with integer is working
     assertion : [as binary 1 = append b 1]

✅ - assert that appending binaries with integer is working
     assertion : [as binary 1 = b ++ 1]

✅ ~ assert that appending binaries with integer returns a binary
     assertion : [binary? append b 1]

✅ - assert that appending binaries with integer returns a binary
     assertion : [binary? b ++ 1]

⏩ - assert that split is working for windows's paths      
      skipped!
```

> [!NOTE]
> Property-based tests have `~` as separator. 

### The *Runner*

Basically, you can run your *tests units* without a *runner*. 
But there are some reasons why you should prefer to use a `runTests` function to run them.

First, your *runner*'s output will give you important information about the current run.
This will show you the file being runned, 
the tests's status
and at the end a summary of failed, skipped and passed tests:

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

### *Runner*
- `runTests: $[folder :string]`:
    The *runner function*, this function will look for *tests* inside the relative `folder`.
    Remember that all *tests* must begin with the `test` prefix, and be an `.art` extension.
    - `.failFast`:
        Fails on the first error found. 
        This works at file scope due to our current way of running tests.
    - `.pattern :string`:
        Defines what is a test-file via a kind-of *glob* pattern.
        Use a `*` as spliter. 
        - Obs.: That is a kind-of *glob* pattern, not a real one. 
          So just use one and only one `*` to split the pre and suffix.
    - `.suppress`: 
        Suppress `panic`, this means: 
        this won't terminate your tests, 
        won't return an error code
        and won't print a `panic` message. 

### *Tests*
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
- `assert: $[condition :block]`:
    A function that is only available inside the `test` case,
    makes an assertion given the `condition`.
- `suite: $[description :string tests :block]`:
    Visually groups tests together.


> [!WARNING]
> Never import this lib as `.lean`, or this will break the current code.
> This happens due to the nature of Arturo (being concatenative), 
> and the way we importings are working right now.
> This may change in future.

---

> Background photo on ["At a Glance"](#at-a-glance) 
  by [Jack Anstey](https://unsplash.com/@jack_anstey?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash) on [Unsplash](https://unsplash.com/photos/aerial-photography-of-road-zS4lUqLEiNA?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash)