"""
Keep track of how many times function has been called
and with what arguments.
"""
import logging


def my_logger(original_function):
    logging.basicConfig(filename='{}.log'.format(original_function.__name__), level=logging.INFO)

    def wrapper(*args, **kwargs):
        logging.info(
            'Ran with args: {}, and kwargs: {}'.format(args, kwargs))
        return original_function(*args, **kwargs)

    return wrapper


@my_logger
def display_info(name, age):
    print('display_info ran with arguments: ({}, {})'.format(name, age))

display_info('Mat', 10)
display_info(age=11, name='Pat')
