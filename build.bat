@echo off
setlocal enabledelayedexpansion

set BINARY_NAME=CapybaraScan

REM Compila o projeto
echo Compilando o projeto em modo release...
cargo build --release

echo.
echo Copiando binario para a pasta dist...

REM Cria a pasta dist se não existir
echo Preparando pasta dist...
if not exist dist (
  mkdir dist
)

REM Copia o binário para dist
copy /Y target\release\%BINARY_NAME%.exe dist\%BINARY_NAME%.exe

echo.
echo Executando o programa...
dist\%BINARY_NAME%.exe

echo.
pause