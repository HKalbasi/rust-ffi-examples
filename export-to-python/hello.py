from rust_ext import axpy
import numpy as np

x = np.random.rand(10)
y = np.random.rand(10)
a = 42

print(x, y)

z = axpy(a, x, y)

print(z)
