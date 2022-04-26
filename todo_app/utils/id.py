import random
import time


def create_id() -> int:
    return int(time.time()) << 32 | random.randint(0, 0xFFFFFFFF)
