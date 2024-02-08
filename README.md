<!-- Don't fill out comments column! -->
# LeetCode problems in Rust

This repository contains my solutions to LeetCode problems in Rust.

## Environment

All problems are solved in VS Code using the [LeetCode extension].

## Results

| # | Title | Solution | Difficulty | Speed score | Memory score | Comments |
|---| ----- | -------- | ---------- | ----------- | ------------ | -------- |
|1|[Two Sum](https://leetcode.com/problems/two-sum/)|[Rust](./src/easy/1.two-sum.rs)|Easy| 23.84 % | 68.85 % | Two pointers: one at $i$ and other at $j>i$ |
|9|[Palindrome Number](https://leetcode.com/problems/palindrome-number/)|[Rust](./src/easy/9.palindrome-number.rs)|Easy| 69.35 % | 17.69 % | Split into digits and converge towards center |
|13|[Roman to Integer](https://leetcode.com/problems/roman-to-integer/)|[Rust](./src/easy/13.roman-to-integer.rs)|Easy| 100 % | 16.15 % | Use mapping for combinations 🤷 |
|14|[Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/)|[Rust](./src/easy/14.longest-common-prefix.rs)|Easy| 100 % | 55.24 % | Filter remaining strings for containing characters from first one |
|20|[Valid Parentheses](https://leetcode.com/problems/valid-parentheses/)|[Rust](./src/easy/20.valid-parentheses.rs)|Easy| 50.15 % | 19.94 % | Stack |
|21|[Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)|[Rust](./src/easy/21.merge-two-sorted-lists.rs)|Easy| 6.97% | 35.95% | Make list of ordered nodes and collapse it from the end |
|26|[Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)|[Rust](./src/easy/26.remove-duplicates-from-sorted-array.rs)|Easy| 100 % | 78.6 % | Make list with uniques and then replace first $n$ entries|
|27|[Remove Element](https://leetcode.com/problems/remove-element/)|[Rust](./src/easy/27.remove-element.rs)|Easy| 79.48 % | 46.72 % | Use two pointers (from start and end) |
|35|[Search Insert Position](https://leetcode.com/problems/search-insert-position/)|[Rust](./src/easy/35.search-insert-position.rs)|Easy| 78.53 % | 66.75 % | Check when difference with target changes signs |
|58|[Length of Last Word](https://leetcode.com/problems/length-of-last-word/)|[Rust](./src/easy/58.length-of-last-word.rs)|Easy| 100 % | 40.44 % | Reset counter on whitespace, except for the last character |
|66|[Plus One](https://leetcode.com/problems/plus-one/)|[Rust](./src/easy/66.plus-one.rs)|Easy| 79.3 % | 73.02 % | Split number into digits and do carry overs for digits preceeding urrent index |
|67|[Add Binary](https://leetcode.com/problems/add-binary/)|[Rust](./src/easy/67.add-binary.rs)|Easy| 26.51 % | 26.05 % | Similar to the 66 |
|69|[Sqrt(x)](https://leetcode.com/problems/sqrtx/)|[Rust](./src/easy/69.sqrtx.rs)|Easy| 100 % | 74.03 % | Newton's method for calculating square root |
|70|[Climbing Stairs](https://leetcode.com/problems/climbing-stairs/)|[Rust](./src/easy/70.climbing-stairs.rs)|Easy| 100 % | 37.6 % | Recurrence @ $n-1$ and $n-2$ for $n>1$ (for $n\in\lbrace1,2\rbrace$ return $n$) + memo with HashMap |
|83|[Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list/)|[Rust](./src/easy/83.remove-duplicates-from-sorted-list.rs)|Easy| 9.48 % | 88.79 % | List with unique entries and then wrap the values from end |
|88|[Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)|[Rust](./src/easy/88.merge-sorted-array.rs)|Easy| 83.65 % | 47.4 % | Three pointers (for $n$, $m$ and $n+m$) |
|94|[Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/)|[Rust](./src/easy/94.binary-tree-inorder-traversal.rs)|Easy| 100 % | 38.52 % | Recursion while enforcing the left-root-right order + clone the nodes accessed as refs and borrowed from RefCells |
|100|[Same Tree](https://leetcode.com/problems/same-tree/)|[Rust](./src/easy/100.same-tree.rs)|Easy| 100 % | 91.67 % | Recursion with match that covers cases for `node_1` and `node_2` (etc. `(None,None)` ) that does triple boolean expression comaring lefts, roots and rights |
|101|[Symmetric Tree](https://leetcode.com/problems/symmetric-tree/)|[Rust](./src/easy/101.symmetric-tree.rs)|Easy| 100 % | 29.84 % | Check if (L,R) is `(None,None)` or `(Some,Some)` with equal L and R's ($L.R=R.L \land L.L=R.R$) |
|104|[Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/)|[Rust](./src/easy/104.maximum-depth-of-binary-tree.rs)|Easy| 100 % | 13.27 % | Create vector of ints with depths for each branches and stop at `None` with pushing the current_depth |
|108|[Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/)|[Rust](./src/easy/108.convert-sorted-array-to-binary-search-tree.rs)|Easy| 72.86 % | 37.14 % | Recursion, with values split at middle index for left and right branches + calculating middle index w.r.t. to evenness of `nums` length (substract 1 if even) |
|110|[Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/)|[Rust](./src/easy/110.balanced-binary-tree.rs)|Easy| 100% | 16.98% | Height for branch is 1 + max of $H(l)$ and $H(r)$, their differences must be <= 1 **AND** the $l$ and $r$ must also be balanced (recursion) |
|111|[Minimum Depth of Binary Tree](https://leetcode.com/problems/minimum-depth-of-binary-tree/)|[Rust](./src/easy/111.minimum-depth-of-binary-tree.rs)|Easy| 58.24 % | 6.59 % | Recursion with running `counter` for branch, if two branches are present - choose the minimum counter value, if at `None` return `counter - 1` |
|112|[Path Sum](https://leetcode.com/problems/path-sum/)|[Rust](./src/easy/112.path-sum.rs)|Easy| 65.6 % | 28.8 % | At each node check if there are available paths on left and/or right. If one is available, go down that path only with recursion, if both - check both and return OR of their respective recursion calls. If there are no paths left, check if `sum == target` (there is nowhere left to go). At each step increment sum by `val` |
|118|[Pascal's Triangle](https://leetcode.com/problems/pascals-triangle/)|[Rust](./src/easy/118.pascals-triangle.rs)|Easy| 84.02 % | 58.9 % | Use previous row entries from `0` to `len - 1` to calculate inner numbers of next row, append `1`s to its start and end. First row is always `[1]`, second one is always `[1,1]`. Use running row counter.|
|119|[Pascal's Triangle II](https://leetcode.com/problems/pascals-triangle-ii/)|[Rust](./src/easy/119.pascals-triangle-ii.rs)|Easy| 100 % | 47.78 % | Same as 118, but only return the last row |
|121|[Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)|[Rust](./src/easy/121.best-time-to-buy-and-sell-stock.rs)|Easy| 89.95 % | 68.92 % | Keep track of minimum price and maximum profit ([greedy algorithm]) |
|125|[Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)|[Rust](./src/easy/125.valid-palindrome.rs)|Easy| 100 % | 34.21 % | Iterate over characters forwards and backwards, while skipping non-alphanumeric characters (while also advancing index), and compare their lowercase ASCII representations|
|136|[Single Number](https://leetcode.com/problems/single-number/)|[Rust](./src/easy/136.single-number.rs)|Easy| 81.45 % | 62.16 % | Iterate and keep track on seen numbers, if a number is seen twice - remove it from cache - the result is the remaining entry (there is also a [trick with XOR operator]): `result ^= number` |
|141|[Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/)|NaN|Easy| NaN | NaN | **NOT AVAILABLE IN RUST** (due to difficulties with multi-pointers in Rust i.e. borrowing rules) |
|144|[Binary Tree Preorder Traversal](https://leetcode.com/problems/binary-tree-preorder-traversal/)|[Rust](./src/easy/144.binary-tree-preorder-traversal.rs)|Easy| 100 % | 53.97 % | Recursion with `root` -> `left` -> `right` order (the `None/Some` letf-right schtick) |

[LeetCode extension]: https://marketplace.visualstudio.com/items?itemName=LeetCode.vscode-leetcode
[greedy algorithm]: https://www.youtube.com/watch?v=HzeK7g8cD0Y
[trick with XOR operator]: https://www.youtube.com/watch?v=fA_jFOhjvDM&embeds_referring_euri=https%3A%2F%2Fleetcode.com%2F&source_ve_path=MjM4NTE&feature=emb_title
