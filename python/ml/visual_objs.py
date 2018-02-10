from functools import singledispatch
from random import randrange
from typing import Tuple, Union, Callable

import pygame as pg
from um.visuals import color

FILL = 0


@singledispatch
def trans_y(y: Union[int, float, Tuple[Union[int, float], Union[int, float]]], surface: pg.Surface) -> Union[int, float]:
    """
    Transpose Y coordinate so it can be displayed as Y = 0 was on the bottom of the surface.

    :param y:
    :param surface: surface on which coordinate is to be put on
    :return: output coordinate
    """
    return surface.get_height() - y


@trans_y.register(tuple)
def __(pos: Tuple[int, int], surface: pg.Surface) -> Tuple[Union[int, float], Union[int, float]]:
    return pos[0], surface.get_height() - pos[1]


class Circle:
    def __init__(self, pos: Tuple[Union[int, float], Union[int, float]]=None, r: Union[int, float]=1, screen: pg.Surface=None):
        if pos is None:
            pos = (0, 0)
        self.pos = pos
        self.r = r
        self.screen = screen
        self.cls = False

    @property
    def x(self):
        return self.pos[0]

    @property
    def y(self):
        return self.pos[1]

    def draw(self):
        # position, Y = 0 is on the bottom on the window
        # pos = (self.pos[0], self.screen.get_height() - self.pos[1])
        pos = trans_y(self.pos, self.screen)
        # outline
        pg.draw.circle(self.screen, color.BLACK, pos, self.r, 1)

        # fill
        pg.draw.circle(
            self.screen,
            color.GREEN if self.cls else color.RED,
            pos,
            self.r - 1, FILL
        )

        return self

    @staticmethod
    def random_circle(screen: pg.Surface, radius: Union[int, float]=1) -> 'Circle':
        x, y = randrange(0, screen.get_width()), randrange(0, screen.get_height())
        return Circle((x, y), radius, screen)

    def is_above_line(self, line: 'Line') -> bool:
        return True if self.y > line(self.x) else False


class Line:
    """
    Create line

    :param a: slope
    :param b: intercept
    """
    def __init__(
            self, a: Union[int, float]=1, b: Union[int, float]=0, screen: pg.Surface=None,
            color_: Tuple[int, int, int]=None, width: Union[int, float]=1
    ):
        self.a = a
        self.b = b
        self.screen = screen
        if color_ is None:
            color_ = color.BLACK
        self.color = color_
        self.width = width

    @property
    def f(self):
        return lambda x: self.a * x + self.b

    def draw(self):
        start = trans_y((0, self.f(0)), self.screen)
        w = self.screen.get_width()
        end = trans_y((w, self.f(w)), self.screen)
        pg.draw.line(self.screen, self.color, start, end, self.width)

        return self

    def __call__(self, *args, **kwargs):
        """
        When instance gets called compute function result.
        """
        return self.f(*args)

