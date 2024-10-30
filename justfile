prod:
	rm -rf /var/www/dom.elayar.fr
	cp -rpv ./dist /var/www/dom.elayar.fr

clean:
	cargo clean
	rm -rf ./dist


