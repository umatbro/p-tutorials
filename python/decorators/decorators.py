#!/usr/bin/env python3

"""
decorator is a function that takes another function as an argument,
adds some kind of functionality and then returns another function,
all this without altering code of function that was passed in
"""

def decorator_function(original_function):
    def wrapper_function(*args, **kwargs):
        print('Wrapper executed this before {}'.format(original_function.__name__))
        return original_function(*args, **kwargs)
    return wrapper_function


class DecoratorClass(object):
    """
    Decorators can also be created with classes
    """
    def __init__(self, original_function):
        self.original_function = original_function

    def __call__(self, *args, **kwargs):
        print('Call method from DecoratorClass executed this before {}'.format(self.original_function.__name__))
        return self.original_function(*args, **kwargs)


@decorator_function
def display():
    print('display function ran')


@decorator_function
def display_info(name, age):
    print('display_info run with arguments ({}, {})'.format(name, age))


@DecoratorClass
def d_i(info):
    print('I print info: {}'.format(info))

# display()
display_info('Mat', 5)
d_i('My info')
