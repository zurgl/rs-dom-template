prod:
	rm -rf /var/www/dom.elayar.fr
	cp -rpv ./dist /var/www/dom.elayar.fr

clean:
	rm -rf /var/www/dom.elayar.fr
	cargo clean


