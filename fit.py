#!/usr/bin/env python3

import time

import numpy as np
from scipy.optimize import minimize

from rustlib import cost_function as rust_cost_function

c1 = 0.1
c2 = 0.5
x_values = np.arange(0, 2e6)
data = c1 * np.sin(x_values) + c2 * np.cos(x_values)
guess = (1.0, 1.0)
num_times_to_run = 1

def python_cost_function(x0):
    c1 = x0[0]
    c2 = x0[1]
    func = c1 * np.sin(x_values) + c2 * np.cos(x_values)
    diff = data - func
    diff = np.sum(diff**2)
    return diff

python_times = []
for _ in range(num_times_to_run):
    start_time = time.time()
    result = minimize(python_cost_function, guess, tol=1e-10)
    t = time.time() - start_time
    python_times.append(t)
print(f"python result: {result}")
print(f"average python time taken: {np.mean(python_times)}")

rust_times = []
l = lambda x: rust_cost_function(data, x)
for _ in range(num_times_to_run):
    start_time = time.time()
    # result = minimize(rust_cost_function, guess, tol=1e-10)
    result = minimize(l, guess, tol=1e-10)
    t = time.time() - start_time
    rust_times.append(t)
print(f"rust result: {result}")
print(f"average rust time taken: {np.mean(rust_times)}")
