#!/usr/bin/env python3

"""
first class function - when you can treat function as any other object or variable
"""

def square(x):
    return x**2

def cube(x):
    return x**3

# assign function to a variable
f = square  # note - now f stores the square function
# f can now be called like a regular function
print(f(5))

# those prints have the same result - both "variables" are functions
print(square)
print(f)

# functions in Python can be passed as arguments to other functions
# or even returned as another function's result
def my_map(func, arg_list):
    """my_map runs func on all arguments from arg_list and returns a list
    with calculated results
    :return list containing results of func on all aruguments from arg_list"""
    result = []
    for arg in arg_list:
        result.append(func(arg))
    return result

squares = my_map(cube, [1,2,3,4,5])
print(squares)

# return function from another function
def logger(msg):

    def log_message():
        print('Log:', msg)

    return log_message

log_hi = logger('Hi!')
log_hi()

#
def html_tag(tag):
    """:return function that encloses given text between tags provided as this function's argument
    :param tag that will be used in returned function"""
    def wrap_text(msg):
        print('<{0}>{1}</{0}>'.format(tag, msg))

    return wrap_text

print_h1 = html_tag('h1')
print_h1('Test headline!')
print_h1('Another headline')

print_paragraph = html_tag('p')
print_paragraph('This is paragraph')
