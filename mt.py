import sys
import random

text = sys.argv[1]

result = ""

text_array = []

for i in range(len(text)):
    if random.randint(0,1) == 0:
        result = result + text[i].upper()
    else:
        result = result + text[i].lower()

print(result)