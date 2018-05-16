import numpy as np


def diagonals(shape):
    return np.flip(np.eye(shape[1]), 0) + np.eye(shape[1])


def random_normal(shape):
    x = np.random.randn(shape[1], shape[0]) - 0.5
    x[x > 0] = 1
    x[x <= 0] = 0
    return x


def vertical(shape):
    x = np.zeros((shape[1], shape[0]))
    x[:, x.shape[1]//2-1:x.shape[1]//2+1] = 1
    return x


def horizontal(shape):
    x = np.zeros((shape[1], shape[0]))
    x[x.shape[0]//2-1:x.shape[0]//2+1, :] = 1
    return x


def cross(shape):
    return (vertical(shape) + horizontal(shape)).astype(bool)


TYPES = {
    'random': random_normal,
    'diagonals': diagonals,
    'vertical': vertical,
    'horizontal': horizontal,
    'cross': cross,
}

