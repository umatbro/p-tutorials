"""
Based on https://appliedgo.net/perceptron/
"""

import numpy as np
from typing import Union, Callable

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

        result = self.bias
        for input, weight in zip(inputs, self.weights):
            result += input * weight

        return sigmoid(result)
        # return int(not signbit(result))
        # return heaviside(sum, self.weights[-1])

    def adjust(self, inputs, target, learning_rate):
        delta = target - self.output(inputs)
        for i, input in enumerate(inputs):
            self.weights[i] += input * delta * learning_rate

        self.bias += delta * learning_rate

    def guess_y(self, x):
        return self.a * x + self.b

    @property
    def a(self):
        return - self.weights[0] / self.weights[1]

    @property
    def b(self):
        return - self.bias / self.weights[1]

    def __len__(self):
        """
        Number of inputs that perceptron can take.
        """
        return len(self.weights)

    def __str__(self):
        return 'Weights: {}\nBias: {}'.format(self.weights, self.bias)


def f(x: float, a: float=None, b: float=None) -> float:
    """
    Linear function.

    :param x: argument
    :param a: slope
    :param b: intercept
    :return: value being the product of a function
    """
    a = 0.5 if a is None else a
    b = 0.1 if b is None else b
    return a * x + b


def is_above_line(x: float, y: float, f: Callable[float, float]) -> bool:
    return y > f(x)


def train(perceptron: Perceptron, n_iters: int, func: Callable[float, float], rate):
    """
    Generate random test points and feed them to perceptron.
    Then compare answer against the solution.
    Change perceptron weights accordingly using Perceptron.adjust function.
    """
    for i in range(n_iters):
        point = np.random.rand(2) * 200 - 100
        target = int(is_above_line(point[0], point[1], func))

        perceptron.adjust(point, target, rate)


def verify(perceptron: Perceptron, func: Callable[float, float]) -> int:
    """
    Verify the percentage of correct perceptron answers.
    Function runs 100 tests and checks correctness.

    :param perceptron: perceptron to be verified
    :param func: linear function
    :return: Percentage of correct answers (0 - 100)
    """
    correct = 0
    for i in range(100):
        point = np.random.rand(2) * 200 - 100

        result = perceptron.output(point)
        if result == is_above_line(point[0], point[1], func):
            correct += 1

    return correct


def sigmoid(x):
    """
    Result calculated as :math:`\frac{1}{1 + e^{-x}}`

    :return: Computed sigmoid function value
    """
    return 1 / (1 + np.e ** (-x))


if __name__ == '__main__':
    # a = np.random.uniform(-5, 5)
    # b = np.random.uniform(-50, 50)
    # perc = Perceptron(2)
    # print(perc)
    # iterations = 10000
    # learn_rate = 0.1
    # f = lambda x: 2 * x + 5
    # train(perc, iterations, f, learn_rate)
    # succ_rate = verify(perc, f)
    #
    # print('{}% succ rate'.format(succ_rate))
    #
    # w0 = perc.weights[0]
    # w1 = perc.weights[1]
    # w2 = perc.bias
    # print('a: {}\tb: {}'.format(-w0/w1, -w2/w1))
    p = Perceptron(2)
    func = lambda x: 1 * x + 0
    print('Before training')
    print('{}%\na: {}\tb: {}'.format(verify(p, func), p.a, p.b))
    train(p, 1000, f, 0.9)
    print('After training')
    print('{}%\na: {}\tb: {}'.format(verify(p, func), p.a, p.b))
    print('Guess f(2) = {}'.format(p.guess_y(2)))
