@echo off
:loop
prime.exe
if %errorlevel% equ 78498 echo PASS
goto loop