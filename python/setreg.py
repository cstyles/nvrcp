#!/usr/bin/env python3

from pynvim import attach
import os
import fileinput

nvim = attach('socket', path=os.environ['NVIM_LISTEN_ADDRESS'])
stdin = ''.join(line for line in fileinput.input())
nvim.funcs.setreg('"', stdin)
