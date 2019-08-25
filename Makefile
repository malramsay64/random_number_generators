#
# Makefile
# Malcolm Ramsay, 2019-08-23 14:35
#

all: main.pdf

main.pdf: main.tex slides.tex
	tectonic $<

slides.tex: slides.md
	pandoc -t beamer $< -o $@


# vim:ft=make
#
