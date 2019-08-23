#
# Makefile
# Malcolm Ramsay, 2019-08-23 14:35
#

all: slides.pdf


slides.pdf: slides.md
	pandoc -t beamer $< -o $@



# vim:ft=make
#
