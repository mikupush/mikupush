.PHONY: build-macos


build-macos:
	bash scripts/build-macos-helpers.sh && bash scripts/build-release.sh
