# https://www.codewars.com/kata/596e4ef7b61e25981200009f/train/python

# solution 1: 652ms
from typing import Tuple
from math import ceil

def aspect_ratio(x: int, y: int) -> Tuple[int, int]:
    return (ceil(y * 16 / 9), y)
