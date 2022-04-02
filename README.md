# fuzzy-neon

**fuzzy-neon:** Collection of fuzzy string matching algorithms written in rust

Created using [neon](https://neon-bindings.com/).

## Installation
**Node**
```sh
$ npm i fuzzy-neon
```

## Usage
```js
const fuzzy = require('fuzzy-neon');
let score = fuzzy.hamming('nick', 'nice');
console.log(score);
// 1
```
## Available String Matching Metrics
 * Hamming distance `hamming(str1, str2)`
 * Levenshtein (recursive impl) `levenshtein(str1, str2)`
 * Wagner-Fischer (dynamic programming impl of levenshtein) `wagnerFischer(str1, str2)`
 * Damerau-Levenshtein `damerauLevenshtein(str1, str2)`
 * Jaro-Winkler Distance `jaroWinkler(str1, str2)`
 * Longest Common Subsequence `lcs(str1, str2)`
 * n-gram based distance metric `ngram(str1, str2, ngramSize)`
 * Jensen-Shannon Vector Distance `jensonshannonVector(str1, str2)`

## Algorithm Explanation/Useful Links
#### Hamming
Computes number of positions between two strings where characters differ.
Expanded to allow strings of different lengths
**Example**
```js
const fuzzy = require('fuzzy-neon');
let score = fuzzy.hamming('nick', 'nice');
console.log(score);
// 1
```
#### Levenshtein
The Levenshtein distance between two strings is the minimum number of single-character edits (insertions, deletions or substitutions) to convert one word to the other. For efficient implementation **use Wagner-Fischer**
**Example**
```js
const fuzzy = require('fuzzy-neon');
let score = fuzzy.levenshtein('nick', 'nit');
console.log(score);
// 2
```
#### Wagner-Fischer
Implementation of Levenshtein using a faster, dynamic programming implementation - interesting to compare performance between the two.
**Example**
```js
const fuzzy = require('fuzzy-neon');
let score = fuzzy.wagnerFischer('nick', 'nit');
console.log(score);
// 2
```
#### Damerau-Levenshtein
Extension of the Levenshtein distance metric which allows for adjacent character transpositions
**Example**
```js
const fuzzy = require('fuzzy-neon');
let score = fuzzy.damerauLevenshtein('taco', 'atco');
console.log(score);
// 1
```
#### Jaro Winkler Distance
Extension of the Jaro distance between two strings; the Jaro distance uses the relative probability of each character in a string to calculate an edit score between two strings (see [here](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance) for formula details). Winkler extended this to boost the scores of words which shared prefixes of some length. Returns a normalised score between 0 and 1.
**Example**
```js
const fuzzy = require('fuzzy-neon');
let score = fuzzy.jaroWinkler('nice', 'nick');
console.log(score);
// 0.11549999999999994
```
#### N-gram Based distance metric
n-gram based string distance metric implemented based on the work from [this paper](http://webdocs.cs.ualberta.ca/~kondrak/papers/spire05.pdf).
Extremely good at integrating context into producing the metric score, O(n^2) complexity.
**Example**
```js
const fuzzy = require('fuzzy-neon');
let score = fuzzy.ngram('rupert', 'robert', 2); // last arg is size of ngram
console.log(score);
// 0.16666666666666666
```
### Jensen Shannon Distance
Computes the relatively probability of events in the string (events being either single characters or bi-grams) porducing a probabilty vector. Then computes the Jensen Shannon distance metric over the two probability vectors. Produced from the work by Richard Connor et al in [this paper](https://scholar.google.co.uk/citations?view_op=view_citation&hl=en&user=wtJy4BEAAAAJ&sortby=pubdate&citation_for_view=wtJy4BEAAAAJ:EYYDruWGBe4C). O(n) complexity.
**Example**
```js
const fuzzy = require('fuzzy-neon');
let score = fuzzy.jensonshannonVector('hi their', 'hi there');
console.log(score);
// 0.36638698518165513
```
