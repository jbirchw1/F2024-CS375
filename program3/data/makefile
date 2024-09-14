CC = gcc
CFLAGS  = -g -Wall -Wextra
TARGET = gendata
  
all: $(TARGET)
  
$(TARGET): $(TARGET).o
	$(CC) $(CFLAGS)	-o $@ $(TARGET).o

$(TARGET).o:
	$(CC) $(CFLAGS) -c $*.c

clean: 
	rm *.o $(TARGET)