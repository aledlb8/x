echo '@echo off' > bin/x.cmd
echo 'node "%~dp0\..\dist\cli.js" %*' >> bin/x.cmd