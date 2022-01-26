PKG=main
TARG=a.out

.PHONY: all
all: $(TARG)

$(TARG): main.a
	go tool link main.a 

main.a: main.o func_amd64.o
	go tool pack c $@ $^
	go tool pack r /Users/krouton/.goenv/versions/1.15.8/pkg/darwin_amd64/encoding/hex.a $@

main.o: main.go symabis
	go tool compile -symabis symabis -p $(PKG) $<

func_amd64.o: func_amd64.s go_asm.h
	go tool asm -p $(PKG) $<

func_amd64.s: go_asm.h

symabis: *.s
	go tool asm -gensymabis -o $@ $^

go_asm.h: *.go
	go tool compile -asmhdr $@ $^

.PHONY: clean
clean:
	rm -f *.o *.a

.PHONY: nuke
nuke:
	rm -f *.o *.a $(TARG) symabis go_asm.h
