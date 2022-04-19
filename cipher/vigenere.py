#! /usr/bin/env python3
# -*- coding: utf-8 -*-
# vim:fenc=utf-8
#
# Copyright © 2022 matthew <matthew@WINDOWS-05HIC4F>
#
# Distributed under terms of the MIT license.

import itertools as it
import re

"""

"""

def vigenere(plaintext, key):
    plaintext = re.sub("[^a-z]", "", plaintext.lower())
    return "".join(chr((ord(p) + ord(k) - 2*ord('a')) % 26 + ord('a')) for p, k in zip(plaintext, it.cycle(key)))
def un_vigenere(plaintext, key):
    plaintext = re.sub("[^a-z]", "", plaintext.lower())
    return "".join(chr((ord(p) - ord(k) + 26) % 26 + ord('a')) for p, k in zip(plaintext, it.cycle(key)))

