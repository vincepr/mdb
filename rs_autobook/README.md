# rs_autobook
Automatically generates a SUMMARY.md for the rust `mdbook` crate.

- The underlying file structure gets parsed into a Summary Markdown file the mdbook-tool will use for chapters.
```md
# Summary

[Welcome](README.md)

----
* [a_1](a_1.md)
* [a_binary_search_tree](a_binary_search_tree.md)
* [datastructures](datastructures.md)
	* [disjoined_set](datastructures/disjoined_set.md)
	* [dynamic_array](datastructures/dynamic_array.md)
	* [Evaluating](datastructures/Evaluating.md)
	* [hashtable](datastructures/hashtable.md)
	* [linked_list](datastructures/linked_list.md)
	* [nested]()
		* [inside]()
			* [testing](datastructures/nested/inside/testing.md)
	* [nestedonce](datastructures/nestedonce.md)
		* [another](datastructures/nestedonce/another.md)
	* [nestedonce](datastructures/nestedonce.md)
	* [priority_queue](datastructures/priority_queue.md)
	* [queue](datastructures/queue.md)
	* [stack](datastructures/stack.md)
* [datastructures](datastructures.md)
* [nestedonce](nestedonce.md)
* [wanted](wanted.md)
```
- by default the first h1 title (`# someTitle`) gets used as names. 
    - But by passing along the `--usefiles` flag the raw file- and foldernames will be used.
- folders will appear as empty points in the list (`nested` and `inside`)
    - but if there is a markdownfile with the same name as the folder, that file will be used.