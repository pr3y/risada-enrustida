PARAMETER ?= Purei

encode:
	./target/debug/risada-enrustida --encode='$(PARAMETER)'

decode:
	./target/debug/risada-enrustida --decode='$(PARAMETER)'


