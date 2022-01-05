CC=clang++
CFLAGS=-Wall -std=c++17 -g
LFLAGS=

main: main.o
	$(CC) $(CFLAGS) -o main main.o $(LFLAGS)

main.o: main.cpp
	$(CC) $(CFLAGS) -c -o main.o main.cpp

clean:
	rm -f *.o main

