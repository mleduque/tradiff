# tradiff

## Usage

```
tradiff <file1> <file2>
```

## Overview

Shows differences in entries between two weidu TRA files.

It will also print a warning when there are duplicated entries in one of the files.

```
me@localhost:~Faiths_and_Powers$ tradiff 'faiths_and_powers/language/english/HLA.tra' 'faiths_and_powers/language/french/HLA.tra'

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚨 WARN The first file (faiths_and_powers/language/english/HLA.tra) contains duplicated entries
  - 12121212
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

+ Entries in the second file but not in the first file:
  - 61614
− Entries in the first file but not in the second file:
  - 12121212

```

## Caveat

The parsing is what I think weidu accepts
 - ids are @<number> with number possibly being negative (for example `@-1000`).
 - Strings can be enclosed in tildas `~aaa~` or in double-quotes `"aaa"`.
 - An entry is either `@id = <string>` or `@id = <string> <feminine string>`
 - A comment is either `// comment until the end of line` or `/* comment betwenn these, possibly with newlines */`

I'm perfectly OK with being corrected if any of these is wrong (though correcting may take time).

I didn't test what happens if you put multiple entries on a single line in weidu.
