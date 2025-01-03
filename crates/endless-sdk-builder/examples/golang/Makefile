#Copyright © Endless Foundation
# SPDX-License-Identifier: Apache-2.0

clean:
	- rm -rf go.mod go.sum endlessstdlib endlesstypes

test:
	cargo run -p endless-framework release
	cargo run -p endless-sdk-builder -- \
		--language go --module-name endlessstdlib \
		--package-name testing \
		--with-endless-types "../../../../testsuite/generate-format/tests/staged/endless.yaml" \
		--target-source-dir . \
		"../../../framework/endless-framework/build/EndlessFramework"
	- go mod init testing
	go mod edit -replace testing=`pwd`
	go get github.com/endless-labs/serde-reflection/serde-generate/runtime/golang/bcs
	go get github.com/endless-labs/serde-reflection/serde-generate/runtime/golang/serde
	go run stdlib_demo.go

.PHONY: clean test
