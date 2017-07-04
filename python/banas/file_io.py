import random
import sys
import os

test_file = open("test.txt", "wb") # "wb" enables to write to the file, "ab" would append

print(test_file.mode)
print(test_file.name)

# writing bytes to a file
test_file.write(bytes("Write ąłę", "UTF-8"))

test_file.close()

# reading from a file
test_file = open("test.txt", "r+")
text_in_file = test_file.read()
print(text_in_file)

# deleting a file
os.remove("test.txt")
