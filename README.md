rreplace is a rust library designed to streamline string replacements.
It can handle multiple unique replacements and iterates the string only
once. Multiple unique repacements may eclipse one another, therefore
rreplace follows this replacement priority.

-   First match

-   Longest match

rreplace in action {#_rreplace_in_action}
==================

run(&str, Hashmap&lt;&str,&str&gt;) → String

:   `run` takes a string argument to search and a Hashmap of
    replacements. The Key of the hashmap is the sequence to match, the
    Value is the sequence to replace.

``` {.rust}
// Create HashMap to define replacements
let replace: Hashmap<&str, &str> = Hashmap::new();
replace.insert("This", "xxxx");
replace.insert("foo", "foobar");

rreplace::run("This string is foo", r);
// Returns:   "xxxx string is foobar"
```

Complex examples {#_complex_examples}
================

The priority with which rreplace updates strings is shown in several
examples below.

Replace First {#_replace_first}
-------------

``` {.rust}
replace.insert("This string", "xxx");
replace.insert("string", "yyy");

rreplace::run("This string is foo", r);
// Returns:   "xxx is foo"
```

`"This string"` begins matching before `"string"` and therefore takes
replacement priority.

Replace Longest {#_replace_longest}
---------------

``` {.rust}
replace.insert("This string", "yyy");
replace.insert("This", "xxx");

rreplace::run("This string is foo", replace);
// Returns:   "xxx is foo"
```

Both seqences begin matching on the same index, therefore the longer
replacement takes priority.

More Replacemets and Failing {#_more_replacemets_and_failing}
----------------------------

### Eclipsing {#_eclipsing}

``` {.rust}
replace.insert("string is foo", "yyy");
replace.insert("i", "I");

rreplace::run("This string is foo", replace);
// Returns:   "ThIs xxx"
```

`"i"` cannot be replaced within `"string is foo"`. This is because
`"string is foo"` begins matching earlier than the individual `"i"`
within.

### Failing {#_failing}

``` {.rust}
replace.insert("string is X", "yyy");
replace.insert("i", "I");

rreplace::run("This string is foo", replace);
// Returns:   "ThIs strIng Is foo"
```

`"string is X"` is unable to find a complete match and therefore `"i"`
matches with every `i` within the string.
