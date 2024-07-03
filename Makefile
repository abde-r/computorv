build:
	@rustc -o computor main.rs

run: build
	@computor

clean:
	@rm computor
