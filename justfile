deploy:
	rm -rf /var/www/dom.elayar.fr
	cp -rpv ./dist /var/www/dom.elayar.fr

clean:
	cargo clean
	rm -rf ./dist

dev: clean
	pnpm dev

build: clean
	pnpm build

