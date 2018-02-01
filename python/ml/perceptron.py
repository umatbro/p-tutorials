"""
Based on https://appliedgo.net/perceptron/
"""

import numpy as np
from typing import Union

from numpy import sign, signbit, heaviside


class Perceptron:
    def __init__(self, n_inputs=None):
        if not n_inputs:
            n_inputs = 2
        self.weights = np.random.rand(n_inputs)
        self.bias = np.random.rand()

    # # # # # # # # # # # # # # # # # # # # # # # # # # # #
    def output(self, inputs: Union[np.ndarray, list]) -> int:
        """
        Calculate output based on given inputs and weights.

        :param inputs: length of inputs must be the same as length of weights
        :return: perceptron output - False if sum is negative, else True. Results can be converted to integers
        """
        if len(inputs) != len(self):
            raise ValueError('Input size not valid (expected size: {})'.format(len(self)))

        sum = self.bias
        for input, weight in zip(inputs, self.weights):
            sum += input * weight

        return int(not signbit(sum))
        # return heaviside(sum, self.weights[-1])

    def adjust(self, inputs, target, learning_rate):
        delta = target - self.output(inputs)
        for i, input in enumerate(inputs):
            self.weights[i] += input * delta * learning_rate

        self.bias += delta * learning_rate

    def __len__(self):
        """
        Number of inputs that perceptron can take.
        """
        return len(self.weights)

    def __str__(self):
        return 'Weights: {}\nBias: {}'.format(self.weights, self.bias)


def f(x, a=None, b=None):
    """
    Linear function

    :param x: argument
    :param a: slope
    :param b: intercept
    :return: value
    """
    a = 0.5 if a is None else a
    b = 0.1 if b is None else b
    return a * x + b


def is_above_line(x, y, f):
    return y > f(x)


def train(perceptron: Perceptron, n_iters: int, f, rate):
    """
    Generate random test points and feed them to perceptron.
    Then compare answer against the solution.
    :return:
    """
    print(perceptron)
    for i in range(n_iters):
        point = np.random.rand(2) * 200 - 100
        # actual = perceptron.output(point)
        target = int(is_above_line(point[0], point[1], f))

        perceptron.adjust(point, target, rate)
    print(perceptron)


def verify(perceptron: Perceptron, f):
    correct = 0
    for i in range(100):
        point = np.random.rand(2) * 200 - 100

        result = perceptron.output(point)
        if result == is_above_line(point[0], point[1], f):
            correct += 1

    return correct


if __name__ == '__main__':
    a = np.random.uniform(-5, 5)
    b = np.random.uniform(-50, 50)
    perc = Perceptron(2)
    print(perc)
    iterations = 10000
    learn_rate = 0.1
    train(perc, iterations, learn_rate)
    succ_rate = verify(perc, lambda x: a * x + b)

    print('{}% succ rate'.format(succ_rate))
