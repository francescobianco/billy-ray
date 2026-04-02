
push:
	@git add .
	@git commit -m "Billy Ray, Who You Are?" || true
	@git push

publish:
	@git add .
	@git commit -m "Update release $$(make -s dev-version)" || true
	@git push
	@cargo login
	@cargo publish
