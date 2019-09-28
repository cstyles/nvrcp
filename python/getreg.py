#!/usr/bin/env python3

from pynvim import attach
import os

nvim = attach('socket', path=os.environ['NVIM_LISTEN_ADDRESS'])
print(nvim.funcs.getreg('"'), end='')
