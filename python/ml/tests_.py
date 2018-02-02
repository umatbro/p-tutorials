import unittest
from unittest import TestCase
from copy import deepcopy

from ml.perceptron import Perceptron, is_above_line, train, verify


class PerceptronTests(TestCase):
    def test_is_above_line_function(self):
        f = lambda x: x
        self.assertTrue(is_above_line(10, 11, f))
        self.assertFalse(is_above_line(10, 10, f))
        self.assertFalse(is_above_line(10, 9, f))

        f = lambda x: 2 * x + 1
        self.assertTrue(is_above_line(10, 22, f))
        self.assertFalse(is_above_line(10, 21, f))
        self.assertFalse(is_above_line(10, 20, f))

    def test_adjust(self):
        p = Perceptron(2)
        p.weights[0] = 0.5
        p.weights[1] = 0.5
        p.bias = 1
        p1 = deepcopy(p)
        p2 = deepcopy(p)
        p3 = deepcopy(p)
        p4 = deepcopy(p)

        inputs1 = [1, 0]
        inputs2 = [1, 2]
        inputs3 = [2, -1]
        inputs4 = [-2, -1]
        output1 = 1
        output2 = 1
        output3 = 1
        output4 = 0
        self.assertAlmostEqual(p1.output(inputs1), output1, places=0)
        self.assertAlmostEqual(p2.output(inputs2), output2, places=0)
        self.assertAlmostEqual(p3.output(inputs3), output3, places=0)
        self.assertAlmostEqual(p4.output(inputs4), output4, places=0)

        # first adjust single
        p1.adjust(inputs1, 0, 0.1)
        self.assertAlmostEqual(p1.weights[0], 0.4, places=0)
        self.assertAlmostEqual(p1.weights[1], 0.5, places=0)
        self.assertAlmostEqual(p1.bias, 0.9, places=0)

        # then adjust p one by one

    def test_train(self):
        p = Perceptron(2)

        def func(x):
            return 2 * x
        result1 = verify(p, func)
        train(p, 1000, func, 0.2)
        result2 = verify(p, func)
        self.assertGreater(result2, result1)

        print('a: {}\tb: {}'.format(p.a, p.b))

        self.assertAlmostEqual(p.a, 2, places=0)
        self.assertAlmostEqual(p.b, 0, places=0)


if __name__ == '__main__':
    unittest.main()
