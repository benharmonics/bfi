check if the program wraps correctly: cell 29999 should be "left" of
cell 0 and cell 0 should be "right" of cell 29999
_____________________________________________________________________

 ``` | 29996 | 29997 | 29998 | 29999 |   0   |   1   |   2   |  ```
_____________________________________________________________________

+++++++++++++++++++++++++++++++++   write '!' to cell 0
.                                   print '!' from cell 0

the following lines require the program to wrap from 0 to 29999
<                                   move to cell 29999
+++++++++++++++++++++++++++++++++   write '!' to cell 29999
.                                   print '!' from cell 29999

the following lines require the program to wrap from 29999 to 0
>                                   move to cell 0
.                                   print '!' from cell 0 again
