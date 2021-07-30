#!/bin/bash
killall miniserve
miniserve ./static --index index.html &

watchexec -e js,css,html ./runme

