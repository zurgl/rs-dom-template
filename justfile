clean:
	rm -rf ./node_modules
	rm -rf ./target
	rm -rf ./dist

install: clean
	pnpm install

dev: clean install
	pnpm dev

build: clean install
	pnpm build

deploy: build
	rm -rf /var/www/dom.elayar.fr
	cp -rpv ./dist /var/www/dom.elayar.fr
