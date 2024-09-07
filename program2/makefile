CC = gcc
CFLAGS  = -g -Wall -Wextra
TARGET = generate_unsorted_data
  
all: $(TARGET)
  
$(TARGET): generate_unsorted_data.o
	$(CC) $(CFLAGS)	-o $@ generate_unsorted_data.o

generate_unsorted_data.o:
	$(CC) $(CFLAGS) -c $*.c

clean: 
	rm *.o $(TARGET)