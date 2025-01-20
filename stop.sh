kill -9 $(ps -a | grep chip-8-emulator | awk '{print $1}')
