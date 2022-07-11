============= BRAINFUCK: MULTIPLY FUNCTION =============
________________________________________________________
| _________"Open copy.b in a text editor ..."_________ |
|| >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> ||
|| +++++++++[>+++++++++>++++++++++++>++++>+++++++++++ ||
|| >+++++>+++++++++++>++++<<<<<<<-]>--.>++++.>>++.<<- ||
|| -.>----.>--.<<+.+.+++++++++.>>>+.<-.<.>+++++++.+++ ||
|| ++.<.>>>--.<<<.<-----.>>>>++++.<<<<++++.----.>.>>> ||
|| .-.+++++.<<<<.>>+.+++.<.<.-----.>.>+.>>----..<<<.> ||
|| ++++.>>+++.-------.<<---.>>>+++.<<<-.<.<--------.> ||
|| >----.<<++.>>-.<<--.>.>+.-.>.>>>++++++++++.        ||
|| ================================================== ||
|| [-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]  Clear some memory ||
|| <<<<<++++++++[>++++++++<-]>+      Write letter 'A' ||
|| [->+>+>+<<<]           Copy 'A' into cells 5 6 & 7 ||
|| >>+>----         Set cell 6 to 'B' & cell 7 to '=' || 
|| >++++++++++       Set cell 8 to LineFeed (newline) ||
|| <<<<<<<                        Move back to cell 0 ||
||         So we have: (0 0 0 0 'A' B' '=' LF)        ||
||                      ^                             ||
========================================================

Let's get started!

A = 2 ++>
B = 19 +++++++++++++++++++<

========================================================
||   SINCE 2 * 19 = 38 WE'LL BE EXPECTING A * B = &   ||
||         BECAUSE & IS ASCII CHARACTER 38            ||
========================================================

Memory: (A B 0 0)
Cursor on A

Addition is implemented using "successive plus 1" operations so
multiplication will be "successive successive plus 1" operations.

Process:

4 cells: (m n o p)
    invariant: p = Bx(A minus m) plus (B minus n); o = B minus n
    while n is not null
        decrease n
        increase o
        increase p
    loop
    while o is not null
        decrease o
        decrease n
    loop
    decrease m
loop
m = 0; n = B; p = B*A; o=0

Code:

[       while m is not null
  >[    while n is not null
    -   decrease n 
    >+  increase o
    >+  increase p
  <<]   loop
  >[    while o is not null
    -   decrease o
    <+  increase
  >]    loop
  <<-   decrease m
]       loop

>>>>.>.>.<<<.>>>>.  print 'AB=&'                    OUTPUT
<<<<[-]<<<       clearing cell 3 
++             reset cell 0 to 2

Minified:

[>[->+>+<<]>[-<+>]<<-]

>>>>.>.>.<<<.>>>>.  print 'AB=&'                    OUTPUT

Final state:
    Memory: (0 B 0 A*B)
    Cursor: on first cell
    Input: unchanged
    Output: unchanged
     
<<<<[-]<<<       clearing cell 3 
++             reset cell 0 to 2

Optional cleansing:
    Move result to first cell
    Clear second cell

    [>[->+>+<<]>[-<+>]<<-]  mult
    >[-]                    clear second cell
    >>[-<<<+>>>]<<<         move result
    .                       print 2*19 = 38         OUTPUT
