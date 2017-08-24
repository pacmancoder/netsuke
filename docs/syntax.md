Template Language Specification V0.1
========================

Language for generating configuration files.

# Common rules

Only _marked_ blocks should be interpreted by templater as directives
Template directive marker can by any sequence.

Suggested default is: `###`
```
### templater code, until line end.
Some config code ### inline templater code ### oter text
```

Templater code __can__ be multiline, if line have "Line expander" character (`\`) at the end. 
Please use this feature as few as possible to keep config code clean.

# Template language syntax
This section covers the most of template syntax specification.

### Tokens and values
Templater parser should have 3 basic types:
- __Token__ - reserverd word, used by parser to distinguish commands
- __Value__ - string value. Can be created using double quotes (`"value"`) or as result of some parser commands (example: logic command `%X AND %Y => "value"`)
- __Variable__ - Parser variable binding. Use `%variable` notation. 
    - If command waits for a `"value"` - variable can be expanded in-place. 
    - If command waits for variable - it must be passed as independent object
      (example: commnad `SET %variable TO "value"` should operate with variable object, not with substituted text)

### Types
- `TOKEN` - reserved word of template engine
- `"value"` - string value
- `%variable` - variable binding, can be casted to `"value"` type

### String literals
- Strings are `"values"`
- Strings are enclosured by `""`
- String can have escape sequences: `\n,\t,\r,\\,\"`
- Strings and variables can be concatinated: `"hello " %user "!"`
- Result of concatination will be `"value"`


__Notes for the following code listings in this document:__
- `[any things]` - optional data

### IF statement
`IF` token marks the beginning of the `IF` statement.  
```netsuke
IF "value"
...
[ELSE [IF "value"]]
...
ENDIF
```
- `ELSE` statement is optional
- There can be any number of `ELSE` statements
- If statement's `"value"` is non-zero value, templater should insert the following next block until it hit `ENDIF` token or `ELSE` statement
- If statement's argument is zero-value, then following text block must be skipped until next `ELSE/ENDIF` statements
- Implementations of _templater_ should allow nesting of `IF` statements

### Logical Operations
Operations, which can evaluate logical expression and return `"0"` or `"1"` values
```netsuke
IF NOT %X AND (%Y OR %Z)
```

__Note:__ Zero value - `"0"` or empty string

- `Brackets` - syntax is `("value")`
    - Changes the operator precedence, it has the highest priority.
- `NOT` - syntax is `NOT "value"`
    - Returns `"1"` if `"value"` is zero value
    - Returns `"0"` if `"value"` is non-zero value
- `AND` - syntax is `"value" AND "value"`.
    - Returns `"0"` if __any__ of `"values"` are zero value
    - Else returns `"1"`
- `OR` - syntax is `"value OR "value""`
    - Returns `"0"` if __all__ `"values"` are zero values
    - Else returns `"1"`

- All logical operations __must__ produce `"value"` type.
- If logical operation token has found, result `"value"` must be built with Shunting-yard algorithm until the end of the  line (`SET ... TO "value"` and `IF "value"` statements)

__Operation precedence:__  
From the highest:
0. `()`
1. `NOT`
2. `AND`
3. `OR`

### Variable modification commands
- `SET %variable TO "value"`
    - Changes variable mapping to the new value
- `UNSET %variable`
    - Removes variable mapping

__Note:__ reading unassigned variable should always return `"0"` value

### Blocks

```netsuke
[HIDDEN] [RAW] BLOCK "Show progress"
...
IF ...  // will be ignored if block is RAW
...
ENDBLOCK
...
...
INSERT BLOCK "Show progress"
...
```

- `[HIDDEN] [RAW] BLOCK ["value"]`
    - Marks new raw text block with name `"value"`
    - All parser directives __must__ be skipped until `ENDBLOCK` token if block
      is `RAW` type.
    - Raw block name is optional and can be skipped
    - Block can be marked as `HIDDEN`. The text in this block __should not__ be included
    - Blocks can be inserted using `INSERT BLOCK` command
- `ENDBLOCK`
    - Marks end of the block
- `INSERT BLOCK "value"`
    - Inserts content of previously defined block

### OS Communication
- `ENV "value"`
    - reads environment variable with name `"value"` and returns it's value as `"value"`
- `CALL "value"`
    - executes `"value"` on user's shell/cmd and returns _stdout_ as `"value"`

### Printing variables
To print something, let the statement return `"value"`
```netsuke
### SET $X TO "test"
Text ### %X ### other text => Text test other text
### SET $Y TO "aaa"
param = ### $X AND $Y ### => param = 1
```