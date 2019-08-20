#!/usr/bin/python3
import random
import IPython

class __cons__:
    onto = {}

    def __init__(self, x, y=None):
        self._id = str(hash(random.random()))
        self.x = x
        self.y = y
        self.value = (self.x, self.y)
        __cons__.onto[self._id] = self.value

    def __onto__(self, onto):
        return __cons__.onto[self._id].value

    def __repr__(self):
        return f"(cons {self.x} {eval('self.y if self.y else None')})"

    def __next__(self):
        return self.y

    def __iter__(self):
        load = [self.x]
        for onto in __cons__.onto.keys():
            load.append(__cons__.onto[onto])
        return (z for z in load)

def cons(x, y=None):
    if y:
        try:
            assert(isinstance(y, __cons__))
            return __cons__(x, y)

        except AssertionError:
            y = cons(y)
            return __cons__(x, y)
    else:
        return __cons__(x)

if __name__ == "__main__":
    print("g = cons(12, 11)")
    print("h = cons(14, cons(15, 16))")
    print("i = cons(g, h)")
    g = cons(12, 11)
    h = cons(14, cons(15, 16))
    i = cons(g, h)
    print("What happens when you attempt: `list(zip(tuple(i)))`?")
    print("Or perhaps try making an iterator of `i` with `I = iter(i)`.\n")
    IPython.embed()
