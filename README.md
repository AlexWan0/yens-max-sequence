Implementation of [Yen's Algorithm](https://en.wikipedia.org/wiki/Yen%27s_algorithm) for finding the top k most likely sequence generations given the negative log likelihood of each vocab entry at each timestep.

A Python interface is provided in python/yens.py

The input to the yens function is a nested list where row i column j corresponds to the negative log probability of the jth token in the sequence being the ith vocabulary term (e.g. -log prob of the second token being 0 is matrix[0][1]).

Make sure to set binary_path to the correct binary for your system.
