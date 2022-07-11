=========== BRAINFUCK: COPY VALUE FUNCTION ===========

________________________________________________________
| _________"Open copy.b in a text editor ..."_________ |
|| +++++++++[>+++++++++>++++++++++++>++++>+++++++++++ ||
|| >+++++>+++++++++++>++++<<<<<<<-]>--.>++++.>>++.<<- ||
|| -.>----.>--.<<+.+.+++++++++.>>>+.<-.<.>+++++++.+++ ||
|| ++.<.>>>--.<<<.<-----.>>>>++++.<<<<++++.----.>.>>> ||
|| .-.+++++.<<<<.>>+.+++.<.<.-----.>.>+.>>----..<<<.> ||
|| ++++.>>+++.-------.<<---.>>>+++.<<<-.<.<--------.> ||
|| >----.<<++.>>-.<<--.>.>+.-.>.>>>++++++++++.[-]>[-] ||
|| >[-]<   In Cell 0: A = ++++++[<++++++++++>-]<+++++ ||

Let's get started!

Cell 0 has been set to the letter 'A' (ASCII 65)

print 'A': .

print linefeed: >++++++++++.[-]<

copy A (in Cell 0) into Cells 1 and 2

note that Cell 0 is deleted;
copying is a destructive operation!

[     while A is not null
  -   decrease A
  >+  increase 1st next cell
  >+  increase 2nd next cell
  <<  go back to A
]     loop

You can then move Cell 2 into Cell 0
to create a "conservative copy illusion"

>>    go to cell 2 
[     while cell 2 is not null
  -   decrease cell 2
  <<+ increase A
  >>  go back to cell 2
]     loop

Let's see our two copies of A:

<<.   cell 0 contains 'A'
>.    cell 1 contains 'B'
>.    cell 2 is now zero!

============================================================
Let's see that again

Printing a newline & setting up B (ASCII 66) on cell 3:
++++++++++.+[>++++++<-]>

The cursor has been left on 'B'

Minified:

    [->+>+<<]>>[-<<+>>]<<   copy & move
    .>.                     print
