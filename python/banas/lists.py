import random
import sys
import os

grocery_list = ['Juice', 'Tomatoes', 'Potatoes', 'Bananas']
print("First item %s" % (grocery_list[0]))
# alternative way to print above line
print("First item", grocery_list[0])

# printing array elements in a specified range
print(grocery_list[1:3]) # NOTICE! 3 is not included

other_events = ["Wash Car", "Pick up kds", "cash check"]

# adding
to_do_list = [other_events, grocery_list]
print(to_do_list)

# appending elements to list
grocery_list.append('Onions')
print(grocery_list)

# inserting item in a specific index
grocery_list.insert(1, "Pickle")

# removing item
grocery_list.remove("Bananas")

# expected output: Juice, Pickle, Tomatoes, Potatoes, Onions
print(grocery_list)

# sorting
grocery_list.sort()
grocery_list.reverse()

# delete item
del grocery_list[4]
print(grocery_list)

# combining lists together
to_do_list2 = other_events + grocery_list
print(len(to_do_list2))
print(max(to_do_list2))
print(min(to_do_list2))
