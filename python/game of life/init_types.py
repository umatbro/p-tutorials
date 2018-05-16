import numpy as np


def diagonals(shape):
    return np.flip(np.eye(shape[0]), 0) + np.eye(shape[0])


def random_normal(shape):
    x = np.random.randn(*shape)
    x[x > 0] = 1
    x[x <= 0] = 0
    return x


def vertical(shape):
    x = np.zeros(shape)
    x[:, x.shape[0]//2-1:x.shape[0]//2+1] = 1
    return x


def horizontal(shape):
    x = np.zeros(shape)
    x[x.shape[0]//2-1:x.shape[0]//2+1, :] = 1
    return x


def cross(shape):
    return (vertical(shape) + horizontal(shape)).astype(bool)


def square_in_the_middle(shape):
    """4x4 square in the middle"""
    x = np.zeros(shape)
    w, h = x.shape
    x[w//2-2:w//2+2, h//2-2:h//2+2] = 1
    return x


TYPES = {
    'random': random_normal,
    'diagonals': diagonals,
    'vertical': vertical,
    'horizontal': horizontal,
    'cross': cross,
    'square': square_in_the_middle,
}

