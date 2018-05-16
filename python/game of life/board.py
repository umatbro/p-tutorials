from typing import Tuple
import pygame as pg
import numpy as np
from init_types import TYPES
from rules import RULES


class Board:
    def __init__(self, shape: Tuple[int, int], resolution: int=10, start_type='random'):
        self.field = TYPES[start_type](shape[::-1])
        self.resolution = resolution

        lefts = np.arange(0, self.field.shape[1] * resolution, resolution)
        tops = np.arange(0, self.field.shape[0] * resolution, resolution)

        self.lefts, self.tops = np.meshgrid(lefts, tops)

    @property
    def width(self) -> int:
        return self.field.shape[1]

    @property
    def height(self) -> int:
        return self.field.shape[0]

    def neighbours(self, pos: Tuple[int, int]) -> np.ndarray:
        col, row = pos
        return self.field[max(0, row-1):row+2, max(0, col-1):col+2]

    def alive_neighbours(self, pos: Tuple[int, int]) -> int:
        col, row = pos
        self_state = self.field[row, col]
        return self.neighbours(pos).sum() - self_state

    def update(self, rule='conway'):
        sel_rule = RULES[rule]
        next_step_array = np.copy(self.field)
        for index, value in np.ndenumerate(next_step_array):
            if value == 0:  # dead cell
                if self.alive_neighbours(index[::-1]) in sel_rule.to_resurrect:
                    next_step_array[index] = 1
            if value == 1:  # alive cell
                if self.alive_neighbours(index[::-1]) not in sel_rule.to_keep_alive:
                    next_step_array[index] = 0

        self.field = next_step_array
        return self.field

    def display(self, screen):
        """
        Display field

        :param screen: pygame screen
        """
        for left, top, value in zip(np.nditer(self.lefts), np.nditer(self.tops), np.nditer(self.field)):

            pg.draw.rect(
                screen,
                (255 * value, ) * 3,  # color
                pg.Rect(left, top, self.resolution, self.resolution),
            )
