.PHONY: build clean default test test-rs test-py

SHELL:=/bin/bash

LIB_ENTRY_FILE?=rust/src/lib.rs
SOURCE_FILES?=$(shell (test -e rust/src || test -e pyo3/src) && find rust/src pyo3/src -type f)

COMPILER?=cargo
RUSTDOC?=rustdoc
BUILD_FLAGS?=--release
TEST_FLAGS?=--no-default-features -- --nocapture

BASH_COMPLETION_DIR?=/usr/share/bash-completion.d
BIN_DIR?=/usr/bin
DOC_DIR?=/usr/share/doc
MAN_DIR?=/usr/share/man
SHARE_DIR?=/usr/share
DEST_DIR?=

REPO_URL="https://github.com/jnphilipp/bikkuri"


ifdef VERBOSE
  Q :=
else
  Q := @
endif


default: release


clean:
	$(Q)cargo clean
	$(Q)rm -rf "doc/"
	@echo "--- Deleted Rust binaries and documentation"
	$(Q)rm -rf ./build ./dist ./python/bikkuri.egg-info
	$(Q)find . -name __pycache__ -exec rm -rf {} \;
	@echo "--- Deleed __pycache__ and build and dist dirs"


build: ${LIB_ENTRY_FILE} ${SOURCE_FILES}
	$(Q)${COMPILER} build ${BUILD_FLAGS}


test: test-rs test-py


test-py:
	$(Q)pip install -e .
	$(Q)python -m unittest python/tests/*.py


test-rs: build
	$(Q)${COMPILER} test ${TEST_FLAGS}


release: test
	$(Q)echo "Finished building bikkuri.so."


changelog.latest.md:
	$(Q)( \
		declare TAGS=(`git tag --sort=taggerdate`); \
		for ((i=$${#TAGS[@]}-1;i>=0;i--)); do \
			if [ $$i -eq 0 ]; then \
				echo -e "**Version $${TAGS[$$i]}**" >> changelog.latest.md; \
				git log $${TAGS[$$i]} --no-merges --format="  * [%h](${REPO_URL}/commit/%H) %s"  >> changelog.latest.md; \
			elif [ $$i -eq $${#TAGS[@]} ] && [ $$(git log $${TAGS[$$i-1]}..HEAD --oneline | wc -l) -ne 0 ]; then \
				echo -e "**Version $${TAGS[$$i-1]}-$$(git log -n 1 --format='%h')**" >> changelog.latest.md; \
				git log $${TAGS[$$i-1]}..HEAD --no-merges --format="  * [%h](${REPO_URL}/commit/%H) %s"  >> changelog.latest.md; \
			elif [ $$i -le $${#TAGS[@]} ]; then \
				echo -e "**Version $${TAGS[$$i]}**" >> changelog.latest.md; \
				git log $${TAGS[$$i-1]}..$${TAGS[$$i]} --no-merges --format="  * [%h](${REPO_URL}/commit/%H) %s"  >> changelog.latest.md; \
				if [[ $${TAGS[$$i-1]} != *"rc"* ]]; then \
					break; \
				fi; \
			fi; \
			if [ $$i -ne 0 ]; then \
				echo -e "" >> changelog.latest.md; \
			fi; \
		done \
	)
