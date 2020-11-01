#!/usr/bin/env python3

import subprocess, os, base64, sys, json

letters = {
	"capital": "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
	"lower case letter": "abcdefghijklmnopqrstuvwxyz",
	"number": "1234567890"
}

data = {}

for prefix, s in letters.items():
	for letter in s:
		print(prefix, letter)
		os.system("espeak -w sound.wav '" + prefix + " " + letter + "'")
		data[letter] = base64.b64encode(open("sound.wav", "rb").read()).decode("utf-8")
		#os.unlink("sound.wav")

dst = "audio.json"
open(dst, "wt").write(json.dumps(data))
print ("data written to " + dst)
