TARGET = ./bench
VERSION = 1.0.0
GIT_HASH := $(shell git rev-parse --short HEAD || echo 'development')
CURRENT_TIME = $(shell date +"%Y-%m-%d:T%H:%M:%S")
LD_FLAGS = '-s -X main.date=${CURRENT_TIME} -X main.version=${VERSION} -X main.commit=${GIT_HASH}'

.PHONY: clean test check $(TARGET)

.NOTPARALLEL:

$(TARGET):
	$(info ===>  BUILD bench)
	@cargo b --release
	@cp ./target/release/$(TARGET) $(TARGET)

# swagger:
# 	$(info ===>  GENERATE swagger)
# 	@go run github.com/swaggo/swag/cmd/swag init -d ./internal/api,./internal/util -g api.go -o ./api
# 	@mv ./api/docs.go ./internal/api/docs.go

clean:
	$(info ===>  CLEAN)
	@cargo clean
	@rm -f $(TARGET)
	@rm perf.data perf.data.old flamegraph.svg

test:
	$(info ===>  TESTING)
	@cargo clean
	@cargo b --release
	@cargo test

check:
	$(info ===>  CHECKING)
	@cargo clean
	@cargo check