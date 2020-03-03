## install LLVM

```
$ brew install llvm
$ echo 'export PATH="/usr/local/opt/llvm/bin:$PATH"' >> ~/.profile
$ export LDFLAGS="-L/usr/local/opt/llvm/lib"
$ export CPPFLAGS="-I/usr/local/opt/llvm/include"
```
