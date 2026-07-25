EPISODE ?= all

.PHONY: install serve build export check

install:
	npm install

serve:
	npm run dev -- episodes/$(EPISODE).md

build:
	npm run build -- episodes/$(EPISODE).md --out ../dist/$(EPISODE)

export:
	npm run export -- episodes/$(EPISODE).md --output $(EPISODE).pdf

check:
	@set -e; \
	for deck in episodes/*.md; do \
		name=$$(basename "$$deck" .md); \
		echo "Slidev 빌드: $$name"; \
		npm run build -- "$$deck" --out "../dist/$$name"; \
	done
