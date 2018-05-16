import pygame as pg
from board import Board
import sys

field = Board((100, 100), 8)

pg.init()
w, h = field.width * field.resolution, field.height * field.resolution
screen = pg.display.set_mode((w, h))
pg.display.set_caption('Game of life')


while True:
    for event in pg.event.get():
        if event.type is pg.QUIT:
            pg.quit()
            sys.exit(0)
        elif event.type is pg.KEYDOWN and event.key is pg.K_ESCAPE:
            pg.quit()
            sys.exit(0)

    field.update()
    field.display(screen)
    pg.display.flip()
