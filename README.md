# fuzzy-neon

**fuzzy-neon:** Collection of fuzzy string matching algorithms written in rust

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Installing fuzzy-neon

Installing fuzzy-neon requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm. In the project directory, run:

```sh
$ npm install
```

This fully installs the project, including installing any dependencies and running the build.

## Building fuzzy-neon

If you have already installed the project and only want to run the build, run:

```sh
$ npm run build
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./index.node`.

## Exploring fuzzy-neon

After building fuzzy-neon, you can explore its exports at the Node REPL:

```sh
$ npm install
$ node
> require('.').hamming('hi', 'hello')
4
```

## Available Approximate (Fuzzy) string matching algorithms
 * Hamming distance `hamming(str1, str2)`
 * Levenshtein (recursive impl) `levenshtein(str1, str2)`
 * Wagner-Fischer (dynamic programming impl of levenshtein) `wagnerFischer(str1, str2)`
 * Damerau-Levenshtein `damerauLevenshtein(str1, str2)`
 * Jaro-Winkler Distance `jaroWinkler(str1, str2)`
 * Longest Common Subsequence `lcs(str1, str2)`
 * n-gram based distance metric `ngram(str1, str2, ngramSize)`
 * Jensen-Shannon Vector Distance `jensonshannonVector(str1, str2)`


## Learn More

To learn more about Neon, see the [Neon documentation](https://neon-bindings.com).

To learn more about Rust, see the [Rust documentation](https://www.rust-lang.org).

To learn more about Node, see the [Node documentation](https://nodejs.org).
