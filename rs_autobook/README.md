# rs_autobook
Automatically generates a SUMMARY.md for the rust `mdbook` crate.

- The underlying file structure gets parsed into a Summary Markdown file the mdbook-tool will use for chapters.
```md
# Summary

[Welcome](README.md)

----
# Summary

[Welcome](README.md)

----
* [Binary Search Tree](a_binary_search_tree.md)
* [Datastructures](datastructures.md)
	* [Union Find - Disjoint Set](datastructures/disjoined_set.md)
	* [Basic structures](datastructures/dynamic_array.md)
	* [Evaluating Data Structures](datastructures/Evaluating.md)
	* [Hash Table - Hash Map](datastructures/hashtable.md)
	* [Linked List (Singly Linked List)](datastructures/linked_list.md)
	* [nested]()
		* [inside]()
			* [testing](datastructures/nested/inside/testing.md)
	* [nested once](datastructures/nestedonce.md)
		* [antoher nested file](datastructures/nestedonce/another.md)
	* [Priority Queue - (often implemented with a Heap)](datastructures/priority_queue.md)
	* [Queue in C#](datastructures/queue.md)
	* [Basic Stack implementation in C#](datastructures/stack.md)
* [nestedonce](nestedonce.md)
* [Index](wanted.md)
```
- by default the first h1 title (`# someTitle`) gets used as names. 
    - But by passing along the `--usefiles` flag the raw file- and foldernames will be used.
- folders will appear as empty points in the list (`nested` and `inside`)
    - but if there is a markdownfile with the same name as the folder, that file will be used.