all:
	@rustc -o computor main.rs

test: all
	@rustc -o tester tester.rs

clean:
	@rm computor tester

re: clean all