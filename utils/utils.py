"""
Module containing utility functions
"""


def remap(x, in_min, in_max, out_min, out_max):
    """
    Map input value (knowing minimum and maximum values that it can take) to another range

    :param x: value to be mapped
    :param in_min: minimum value of input
    :param in_max: maximum value of input
    :param out_min: minimum value of output
    :param out_max: maximum value of output
    :return: changed value
    """
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min


def constrain(value, max_val, min_val=0):
    """
    Constrain given value in given boundries

    :param value: value to be constrained
    :param max_val: value cannot be more than this
    :param min_val: returned value won't be lower than this
    :return: constrained value
    """
    return max(min_val, min(value, max_val))

