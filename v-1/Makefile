all:
	@rustc -o computor main.rs

build: build
	@computor

test: all
	@rustc -o tester tester.rs

clean:
	@rm computor tester
