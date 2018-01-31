"""
Based on https://appliedgo.net/perceptron/
"""

import numpy as np
from numpy import sign, signbit, heaviside


class Perceptron:
    def __init__(self, n_inputs=None):
        if not n_inputs:
            n_inputs = 2
        self.__weights = np.random.rand(n_inputs + 1)  # + 1 for bias weight
        # self.bias = 1

    @property
    def weights(self):
        return self.__weights[:-1]

    @weights.setter
    def weights(self, value):
        self.__weights[:-1] = value

    @property
    def bias(self):
        return self.__weights[:-1]

    @bias.setter
    def bias(self, value):
        self.__weights[:-1] = value

    # # # # # # # # # # # # # # # # # # # # # # # # # # # #
    def output(self, inputs: list) -> bool:
        """
        Calculate output based on given inputs and weights.

        :param inputs: length of inputs must be the same as length of weights
        :return: perceptron output - False if sum is negative, else True. Results can be converted to integers
        """
        if len(inputs) != len(self):
            raise ValueError('Input size not valid (expected size: {})'.format(len(self)))

        sum = self.bias  # * self.bias  # bias can be commented out as it would always remain 1
        for input, weight in zip(inputs, self.weights):
            sum += input * weight

        return not signbit(sum)
        # return heaviside(sum, self.weights[-1])

    def adjust(self, inputs, target, learning_rate):
        delta = self.output(inputs) - target
        for i, input in enumerate(inputs):
            self.weights[i] = input * delta * learning_rate

        self.bias += delta * learning_rate

    def __len__(self):
        return len(self.weights) - 1

