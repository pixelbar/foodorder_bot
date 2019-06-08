# Foodorder bot

[![Build Status](https://travis-ci.org/pixelbar/foodorder_bot.svg?branch=master)](https://travis-ci.org/pixelbar/foodorder_bot)

This is a foodorder bot for the [Pixelbar](https://www.pixelbar.nl) hackerspace.

## Run

A docker image is generated every release. Please check the [Releases](https://github.com/pixelbar/foodorder_bot/releases) page. You can download the `foodorder_bot.tar` image there. Afterwards, run the following commands:

```
docker load --input foodorder_bot.tar
docker run -d foodorder_bot
```
