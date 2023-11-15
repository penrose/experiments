#!/usr/bin/env bash
wget -nc -O linuxlibertine.tgz https://sourceforge.net/projects/linuxlibertine/files/linuxlibertine/5.3.0/LinLibertineTTF_5.3.0_2012_07_02.tgz/download
mkdir fonts
tar xzf linuxlibertine.tgz -C fonts
