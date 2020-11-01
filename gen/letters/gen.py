#!/usr/bin/env python3

import subprocess, os, base64, sys, json

FNULL = open(os.devnull, "w")
html = open("template.html").read()
fname = "file://" + os.getcwd() + "/index.html"

def clean(fname):
	try:
		os.remove(fname)
	except:
		pass

data = {}

for i in "ABCDEFGHJKMNPQRSTUVWXYZ23456789abcdefghijklmnpqrstuvwxyz":
	print (i)
	clean("screenshot.png")
	open("index.html", "wt").write(html.replace("XXX", i))
	subprocess.call(["google-chrome", "--headless", "--disable-gpu", "--screenshot", fname], stderr = FNULL)
	if not os.path.exists("screenshot.png"):
		print ("error")
		sys.exit(1)
	subprocess.call(["convert", "-trim", "screenshot.png", "screenshot.png"])
	data[i] = base64.b64encode(open("screenshot.png", "rb").read()).decode("utf-8")
	sys.exit(0)
	clean("index.html")
	clean("screenshot.png")

dst = "../src/fonts/font_default.json"
open(dst, "wt").write(json.dumps(data))
print ("data written to " + dst)
