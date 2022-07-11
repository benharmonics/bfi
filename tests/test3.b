You might think this should panic since we add 1 to the first byte 
more than 255 times; it just overflows to zero without a panic

++++++++++++++++++++++++++++++++++++++++++++++++++      50 increments
++++++++++++++++++++++++++++++++++++++++++++++++++      50 increments
++++++++++++++++++++++++++++++++++++++++++++++++++      50 increments
++++++++++++++++++++++++++++++++++++++++++++++++++      50 increments
++++++++++++++++++++++++++++++++++++++++++++++++++      50 increments
+++++++.                                                 6 increments
                                                       ______________
                                                      = 257 increments
                                                       (should overflow
                                                       back to one)
