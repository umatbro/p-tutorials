import random
import sys
import os

# we are not able to change a tuple after it is created
pi_tuple = (3,1,4,1,5,9)

# convert tuple into the list
new_tuple = list(pi_tuple)

# convert list into a tuple
new_list = tuple(new_tuple)

print (pi_tuple)    # tuple
print (new_tuple)   # list
print (new_list)    # tuple
