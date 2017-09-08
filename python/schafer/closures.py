#!/usr/bin/env python3

def outer_func():
    message = 'Hi'

    def inner_func():
        print(message)

    return inner_func

my_func = outer_func()  # my func is now equal to inner_func
print(my_func.__name__)

my_func()
# interestingly function will print 'Hi'.
# my_func() still has access to message variable from outer_func
# So:
"""
A closure is an inner function that remembers and has access to
variables in the local scope in which it was created,
even after the outer function has finished executing.
"""

def out_func_v2(msg):
    """Now time for function with parameters"""
    message = msg

    def inner_func(name=''):
        print(message, name)

    return inner_func

hi_func = out_func_v2('Hi')
hello_func = out_func_v2('Hello')

hi_func('Mat')
hello_func()
