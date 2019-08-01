#!/usr/bin/env python3
from __future__ import annotations
import collections
import IPython
from itertools import permutations
i = IPython.embed

Point = collections.namedtuple('Point', ['x', 'y'])
Rectangle = collections.namedtuple( 'Rectangle',
        [
            'lower_left',
            'lower_right',
            'upper_left',
            'upper_right',
        ]
    )

first = Point(2, 3)
second = Point(2, 1)
third = Point(1, 3)
fourth = Point(1, 1)

r = Rectangle(first, second, third, fourth)

def sort_corners(rectangle: Rectangle) -> Rectangle:
    # There's a chance that (1, 1) is not the true
    # upper right-hand corner, so let's go thru
    # each of the points, and assign them properly
    
    arrangements = permutations(rectangle)
    
    test_lL = lambda r: True if r.lower_left.x < r.lower_right.x and r.lower_left.y < r.upper_left.y else False
    test_lR = lambda r: True if r.lower_right.x > r.lower_left.x and r.lower_right.y < r.upper_right.y else False
    test_uL = lambda r: True if r.upper_left.x < r.upper_right.x and r.upper_left.y > r.lower_left.y else False
    test_uR = lambda r: True if r.upper_right.x > r.upper_left.x and r.upper_right.y > r.lower_right.y else False

    for arr in arrangements:
        lower_left, lower_right, upper_left, upper_right = arr
        arrangement = {
                'lower_left': lower_left,
                'lower_right': lower_right,
                'upper_left': upper_left,
                'upper_right': upper_right }
        rec = Rectangle(**arrangement)
        """
        print(f"rec = {rec}")
        print(f"test_lL = lambda r: True if {r.lower_left.x} < {r.lower_right.x} and {r.lower_left.y} < {r.upper_left.y} else False")
        print(f"{test_lL(rec)}")

        print(f"test_lR = lambda r: True if {r.lower_right.x} > {r.lower_left.x} and {r.lower_right.y} < {r.upper_right.y} else False")
        print(f"{test_lR(rec)}")

        print(f"test_uL = lambda r: True if {r.upper_left.x} < {r.upper_right.x} and {r.upper_left.y} > {r.lower_left.y} else False")
        print(f"{test_uL(rec)}")

        print(f"test_uR = lambda r: True if {r.upper_right.x} > {r.lower_right.x} and {r.upper_right.y} > {r.lower_right.y} else False")
        print(f"{test_uR(rec)}")
        print()
        """
        if all([test_lL(rec), test_lR(rec), test_uL(rec), test_uR(rec)]):
            return rec
        else:
            pass
def calculate_area(rectangle: Rectangle) -> float:
    base = rectangle.lower_right.x - rectangle.lower_left.x
    height = rectangle.upper_left.y - rectangle.lower_right.y
    return base * height

r = sort_corners(r)
print(r)
print("The area of the rectangle is:", calculate_area(r))
