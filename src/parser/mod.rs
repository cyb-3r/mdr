pub mod tokenizer;

/* NOTE:
the parser will need a tokenizer to iterate over tokens
and create the correct html element from parsing.

parsing context should be represented by a stack,
containing state enum members.
once we enter a code block for instance, a new member
representing code block is piled into the stack.
once we leave that same code block, we depile this member
and continue to append element to the previous one.

let's start with something simple:
- lines beginning with # token are headers;
- groups of contiguous lines are paragraphs;
*/
