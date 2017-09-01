IDIR = ../../deps/xpir/
CPP=g++
CFLAGS=-std=c++11 -I. -I$(IDIR) -O3

ODIR=obj
LDIR =../../deps/xpir/_build/pir/
BDIR=bin

LIBS=-L$(LDIR) -lpir_static -lgmp -lmpfr -fopenmp -lboost_thread -lboost_system

_DEPS = pir.hpp
DEPS = $(patsubst %,$(IDIR)/%,$(_DEPS))

_OBJ = pir.o main.o 
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))


$(ODIR)/%.o: %.cpp
	@mkdir -p $(@D)
	$(CPP) -c -o $@ $< $(CFLAGS)

$(BDIR)/main: $(OBJ)
	@mkdir -p $(@D)
	$(CPP) -o $@ $^ $(CFLAGS) $(LIBS)

all: main

.PHONY: clean

clean:
	rm -f $(ODIR)/*.o *~ core $(INCDIR)/*~ 
