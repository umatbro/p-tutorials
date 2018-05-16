import unittest
from board import Board


class TestField(unittest.TestCase):
    def setUp(self):
        """
        x x x x x
        x x o x x
        x o o o x
        x x x o x
        x x x x o
        """
        self.board = Board((5, 5))
        self.board.field.fill(0)
        self.board.field[1, 2] = 1
        self.board.field[2, 1] = 1
        self.board.field[2, 2] = 1
        self.board.field[2, 3] = 1
        self.board.field[3, 3] = 1
        self.board.field[4, 4] = 1

    def test_neighbours(self):
        neighbours = self.board.neighbours((1, 1))
        self.assertListEqual(neighbours.tolist(), [
            [0, 0, 0],
            [0, 0, 1],
            [0, 1, 1],
        ])
        corner_neighbours = self.board.neighbours((4, 4))
        self.assertListEqual(corner_neighbours.tolist() ,[
            [1, 0],
            [0, 1]
        ])

        left_corner_neighbours = self.board.neighbours((0, 0))
        self.assertListEqual(left_corner_neighbours.tolist(), [
            [0, 0],
            [0, 0]
        ])

    def test_count_neighbours(self):
        count_neighbours = self.board.alive_neighbours((2, 2))
        self.assertEqual(count_neighbours, 4)
        cn2 = self.board.alive_neighbours((0, 2))
        self.assertEqual(cn2, 1)
        cn3 = self.board.alive_neighbours((4, 4))
        self.assertEqual(cn3, 1)
        cn4 = self.board.alive_neighbours((2, 3))
        self.assertEqual(cn4, 4)

    def test_update(self):
        self.board.update()
        print(self.board)
