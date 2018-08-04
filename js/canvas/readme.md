[Tutorial](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Drawing_shapes)

# Notes

## Drawing rectangles

``canvas`` only supports one primitive shape: rectangle.

There are 3 functions that draw rectangles on the canvas: 
* ``fillRect(x, y, width, height)`` - draws filled rectangle
* ``strokeRect(x, y, width, height)`` - draws rectangular outline
* ``clearRect(x, y, width, height)`` - clears the specified rectangular area making it transparent

## Drawing paths

Functions to use paths:
* ``beginPath()`` Creates a new path. Once created, future drawing commands are directed into the path and used to build the path up.
* ``moveTo()`` Moves the starting point of a new sub-path to the (x, y) coordinates.
* ``lineTo()`` Connects the last point in the subpath to the x, y coordinates with a straight line.
* [other](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D#Paths) (like ``quadraticCurveTo`` or ``arc`` and ``ellipse``)


# Bezier curves

[Tutorial](http://www.algorytm.org/podstawy-grafiki/krzywa-beziera.html) with formulas.