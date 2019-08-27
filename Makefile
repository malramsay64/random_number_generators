#
# Makefile
# Malcolm Ramsay, 2019-08-23 14:35
#

pandoc_options = --bibliography=bibliography.bib --csl footcite.csl

all: main.pdf

main.pdf: main.tex slides.tex
	tectonic $<

slides.tex: slides.md
	pandoc -t beamer $(pandoc_options) $< -o $@


# vim:ft=make
#
