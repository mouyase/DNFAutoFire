@echo off
set input=%1
certutil -hashfile %input% MD5
pause