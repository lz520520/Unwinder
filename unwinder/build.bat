ml64 /c /Fo gateway.obj src/gateway.asm
lib /out:lib/gateway.lib gateway.obj
rm *.obj
