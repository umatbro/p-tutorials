import unittest
from unittest import TestCase

from ml.perceptron import Perceptron, is_above_line


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


if __name__ == '__main__':
    unittest.main()
