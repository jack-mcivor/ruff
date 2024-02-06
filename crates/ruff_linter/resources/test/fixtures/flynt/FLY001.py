
import secrets
from random import random, choice

# strings that already contained a { or } should not be modified
# ref https://github.com/astral-sh/ruff/issues/5332

a = "Hello"
ok1 = f"{a} World"

nok1 = a + " " + "World"
